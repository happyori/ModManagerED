use std::sync::Arc;

use anyhow::anyhow;
use surrealdb::sql::Thing;
use tauri::AppHandle;

use crate::commands::GameInstanceApi;
use crate::generic;
use crate::manager_error::ModManageredError;
use crate::plugins::database::Database;
use crate::plugins::steamlocate::SteamLocate;
use crate::schema::{GameInstance, GameInstanceDataModel};

const MAIN_INSTANCE: (&str, &str) = ("GameInstances", "main");
impl GameInstanceDataModel {
    fn with_defaults(self, steam_locate: &SteamLocate, handle: &AppHandle) -> Self {
        let path = self.path.or(steam_locate.get_elden_ring_install_str());
        let mod_engine_path = self
            .mod_engine_path
            .or_else(|| {
                let path = handle.path_resolver().resolve_resource("modengine/")?;
                let canonical = dunce::canonicalize(path).ok()?;
                canonical.into_os_string().into_string().ok()
            });

        Self { path, mod_engine_path }
    }
}

#[derive(Clone)]
pub struct GameInstanceApiImpl {
    database: Arc<Database>,
    steam_locate: Arc<SteamLocate>,
}

impl GameInstanceApiImpl {
    pub fn new(database: Arc<Database>, steam_locate: Arc<SteamLocate>) -> Self {
        Self { database, steam_locate }
    }
}

const UPSERT: &str = r"fn::upsert($id, $data)";

#[taurpc::resolvers]
impl GameInstanceApi for GameInstanceApiImpl {
    async fn upsert(self, data: GameInstanceDataModel, app_handle: AppHandle) -> Result<GameInstance, ModManageredError> {
        let data = data.with_defaults(&self.steam_locate, &app_handle);
        let upserted: Option<_> = self
            .database
            .conn
            .query(UPSERT)
            .bind(("id", Thing::from(MAIN_INSTANCE)))
            .bind(("data", data))
            .await?
            .take(0)?;
        let Some(instance) = upserted else {
            generic!("Failed to create or update the game instance")
        };

        Ok(instance)
    }
    async fn fetch(self) -> Result<GameInstance, ModManageredError> {
        Ok(self.database.get_instance().await?)
    }
    async fn update(self, data: GameInstance) -> Result<GameInstance, ModManageredError> {
        let error = anyhow!("Failed to update game instance with id {:#?}", &data.id);
        let updated = self
            .database
            .conn
            .update(&data)
            .content(Into::<GameInstanceDataModel>::into(data))
            .await?
            .ok_or(error)?;

        Ok(updated)
    }
    async fn reset(self, app_handle: AppHandle) -> Result<GameInstance, ModManageredError> {
        let instance = self
            .database
            .conn
            .update(MAIN_INSTANCE)
            .content(GameInstanceDataModel::default().with_defaults(&self.steam_locate, &app_handle))
            .await?
            .ok_or(anyhow!("Failed to reset the game instance"))?;

        Ok(instance)
    }
}