use std::path::PathBuf;

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::auth::Root;
use surrealdb::opt::Config;
use surrealdb::Surreal;
use tauri::{Manager, PathResolver, Runtime};
use tauri::plugin::{Builder, TauriPlugin};
use crate::database_id::DbID;
use crate::plugins::database_trait::DatabaseTrait;
use crate::schema::ModInfo;

#[derive(Debug)]
pub(crate) struct Database {
    pub conn: Surreal<Db>,
}

const ROOT: Root = Root {
    username: "root",
    password: "root"
};

#[async_trait]
impl DatabaseTrait for Database {
    async fn get_active_mods(&self, profile: DbID) -> Result<Vec<ModInfo>> {
        const ACTIVE_MODS_QUERY: &str = "\
            SELECT VALUE ->ProfileMods->ModInfos.*
            FROM ONLY $profile;
        ";

        let response = self
            .conn
            .query(ACTIVE_MODS_QUERY)
            .bind(("profile", profile.0))
            .await?
            .take(0)?;
        Ok(response)
    }

    async fn enable_mod(&self, profile: DbID, mod_info: DbID) -> Result<()> {
        const CHECK_RELATION: &str = "\
            array::any((SELECT id FROM ProfileMods WHERE out = $mod_info AND in = $profile));
        ";
        const RELATE_MOD: &str = "RELATE $profile -> ProfileMods -> $mod_info";
        let relation_exists: Option<bool> = self
            .conn
            .query(CHECK_RELATION)
            .bind(("mod_info", mod_info.0.clone()))
            .bind(("profile", profile.0.clone()))
            .await?
            .take(0)?;
        if relation_exists.unwrap_or(false) {
            bail!("Mod is already enabled")
        }

        self.conn
            .query(RELATE_MOD)
            .bind(("profile", profile.0))
            .bind(("mod_info", mod_info.0))
            .await?;

        Ok(())
    }

    async fn disable_mod(&self, profile: DbID, mod_info: DbID) -> Result<()> {
        self.conn
            .query("DELETE $profile->ProfileMods WHERE out=$mod_info")
            .bind(("profile", profile.0))
            .bind(("mod_info", mod_info.0))
            .await?;

        Ok(())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct DatabaseConfig {
    base_path: Option<PathBuf>,
    database_filename: String,
    database_name: String,
    database_namespace: String,
}

pub fn init<R: Runtime>() -> TauriPlugin<R, DatabaseConfig> {
    Builder::<R, DatabaseConfig>::new("database")
        .setup_with_config(|app, config| {
            let db_path = get_db_path(app.path_resolver(), &config);
            let handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                let db_conn = create_db_connection(db_path, config)
                    .await
                    .expect("Database should have been setup");
                handle.manage(Database { conn: db_conn });
            });
            Ok(())
        })
        .build()
}

fn get_db_path(path_resolver: PathResolver, config: &DatabaseConfig) -> PathBuf {
    let config = config.to_owned();
    let base = config
        .base_path
        .or(path_resolver.app_data_dir())
        .or(path_resolver.app_cache_dir())
        .expect("failed to obtain app data or app cache directory...");
    println!(
        "Selected path for db [{base:#?} + {:#?}]",
        &config.database_filename
    );
    base.join(config.database_filename)
}

async fn create_db_connection(
    path: PathBuf,
    config: DatabaseConfig,
) -> Result<Surreal<Db>> {
    println!(
        "Creating DB connection to {:#?} with: \n{:#?}",
        &path, &config
    );
    let conf = Config::default().user(ROOT);
    let conn = Surreal::new::<RocksDb>((path, conf)).await?;
    conn.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    conn.use_ns(config.database_namespace)
        .use_db(config.database_name)
        .await?;

    Ok(conn)
}