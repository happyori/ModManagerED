use std::sync::Arc;
use tauri::AppHandle;
use crate::commands::ModManagementApi;
use crate::database_id::DbID;
use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::plugins::modengine_manager::ModEngineManager;
use crate::schema::{ModInfo, Profile};

#[derive(Clone)]
pub struct ModManagementApiImpl {
    database: Arc<Database>
}

impl ModManagementApiImpl {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[taurpc::resolvers]
impl ModManagementApi for ModManagementApiImpl {
    async fn enable(self, profile_id: DbID, mod_id: DbID) -> ManagerResult<()> {
        self.database.enable_mod(profile_id, mod_id).await?;
        Ok(())
    }

    async fn disable(self, profile_id: DbID, mod_id: DbID) -> ManagerResult<()> {
        self.database.disable_mod(profile_id, mod_id).await?;
        Ok(())
    }

    async fn fetch_active(self, profile_id: DbID) -> ManagerResult<Vec<ModInfo>> {
        let mods = self
            .database
            .get_active_mods(profile_id)
            .await?;
        Ok(mods)
    }

    async fn launch_game(self, profile: Profile, app_handle: AppHandle) -> ManagerResult<()> {
        //TODO: Add tracing here
        ModEngineManager::launch_modded(&profile, &self.database, app_handle.path_resolver()).await?;
        Ok(())
    }
}