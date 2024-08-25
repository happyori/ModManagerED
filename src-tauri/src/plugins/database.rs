use std::path::PathBuf;

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::opt::{Config, IntoResource};
use surrealdb::Surreal;
use tauri::api::path::{app_cache_dir, app_local_data_dir};
use tracing::{debug, instrument};
use crate::database_id::DbID;
use crate::manager_error::ManagerResult;
use crate::plugins::database_trait::{DatabaseTrait, IntoRefResource};
use crate::schema::{GameInstance, ModInfo};
use tauri::Config as TauriConfig;

#[derive(Debug)]
pub(crate) struct Database {
    pub conn: Surreal<Db>,
}

impl Database {
    #[instrument(skip(tauri_config))]
    pub async fn new(config: DatabaseConfig, tauri_config: &TauriConfig) -> Result<Self> {
        let data_dir = config.base_path
            .clone()
            .or(app_local_data_dir(tauri_config))
            .or(app_cache_dir(tauri_config))
            .ok_or(anyhow!("Database Directory should have been located"))?
            .join(&config.database_filename);

        debug!("Initializing database");

        let connection = create_db_connection(data_dir, config).await?;
        define_upsert(&connection).await?;

        Ok(Self { conn: connection })
    }

    #[instrument]
    pub async fn get_active_mods(&self, profile: DbID) -> Result<Vec<ModInfo>> {
        debug!("Getting currently active mods");
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

    #[instrument]
    pub async fn enable_mod(&self, profile: DbID, mod_info: DbID) -> Result<()> {
        debug!("Enabling mod {:?} for {:?}", &mod_info, &profile);
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

    pub async fn disable_mod(&self, profile: DbID, mod_info: DbID) -> Result<()> {
        debug!("Disabling mod {:?} for {:?}", &mod_info, &profile);
        self.conn
            .query("DELETE $profile->ProfileMods WHERE out=$mod_info")
            .bind(("profile", profile.0))
            .bind(("mod_info", mod_info.0))
            .await?;

        Ok(())
    }

    pub async fn get_instance(&self) -> Result<GameInstance> {
        debug!("Getting main instance");
        let result = self
            .conn
            .select(("GameInstances", "main"))
            .await?
            .ok_or(anyhow!("Could not retrieve main instance"))?;

        Ok(result)
    }
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn create<R, RModel>(&self, resource: impl IntoResource<Vec<R>> + Send, content: RModel) -> ManagerResult<R>
    where
        R: Send + DeserializeOwned,
        RModel: Send + Serialize,
    {
        debug!("Creating a resource");
        let created: Vec<R> = self.conn
            .create(resource)
            .content(content)
            .await?;
        let first = created.into_iter().next().ok_or(anyhow!("Failed to create resource"))?;
        Ok(first)
    }

    async fn update<R, RModel>(&self, content: R) -> ManagerResult<Option<R>>
    where
        RModel: Send + Serialize + From<R>,
        R: Send + DeserializeOwned + IntoRefResource,
    {
        debug!("Updating a resource");
        let update = self
            .conn
            .update(content.into_referenced_resource())
            .content(Into::<RModel>::into(content))
            .await?;

        Ok(update)
    }

    async fn delete<R>(&self, content: R) -> ManagerResult<Option<R>>
    where
        R: Send + IntoResource<Option<R>> + DeserializeOwned,
    {
        debug!("Deleting a resource");
        let deleted = self
            .conn
            .delete(content)
            .await?;

        Ok(deleted)
    }

    async fn fetch<R>(&self, resource: impl IntoResource<Vec<R>> + Send) -> ManagerResult<Vec<R>>
    where
        R: Send + DeserializeOwned,
    {
        debug!("Fetching resources");
        let fetched = self
            .conn
            .select(resource)
            .await?;

        Ok(fetched)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct DatabaseConfig {
    base_path: Option<PathBuf>,
    #[serde(alias = "filename")]
    database_filename: String,
    #[serde(alias = "name")]
    database_name: String,
    #[serde(alias = "namespace")]
    database_namespace: String,
}

#[instrument]
async fn create_db_connection(
    path: PathBuf,
    config: DatabaseConfig,
) -> Result<Surreal<Db>> {
    debug!("Creating db connection to {:#?}", &path);

    let conf = Config::default();
    let conn = Surreal::new::<RocksDb>((path, conf)).await?;

    conn.use_ns(config.database_namespace)
        .use_db(config.database_name)
        .await?;

    debug!("Done");
    Ok(conn)
}

#[instrument]
async fn define_upsert(conn: &Surreal<Db>) -> Result<()> {
    debug!("Defining Upsert function");
    const DEFINE_QUERY: &str = r"
    DEFINE FUNCTION fn::upsert($rec: record, $data: any) {
        LET $id = meta::id($rec);
        LET $tbl = meta::tb($rec);
        LET $exists = !!(SELECT * FROM ONLY type::thing($tbl, $id));
        RETURN IF $exists {
            RETURN UPDATE type::thing($tbl, $id) CONTENT $data RETURN AFTER;
        } ELSE {
            RETURN CREATE type::thing($tbl, $id) CONTENT $data RETURN AFTER;
        }
    }
    ";

    conn.query(DEFINE_QUERY)
        .await?;

    debug!("Done");
    Ok(())
}