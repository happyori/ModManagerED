use anyhow::anyhow;
use surrealdb::sql::Thing;
use tauri::{AppHandle, State};

use macros::define_cmd;

use crate::generic;
use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::plugins::database_trait::DatabaseTrait;
use crate::plugins::steamlocate::SteamLocate;
use crate::schema::{GameInstance, GameInstanceDataModel};

const MAIN_INSTANCE: (&str, &str) = ("GameInstances", "main");

impl GameInstanceDataModel {
    fn with_defaults(self, steam_locate: &SteamLocate, handle: &AppHandle) -> Self {
        Self {
            path: self.path.or(steam_locate.get_elden_ring_install_str()),
            mod_engine_path: self.mod_engine_path.or_else(|| {
                let path = handle.path_resolver().resolve_resource("modengine/").expect("Resource should have resolved");
                let simplified = dunce::canonicalize(path).expect("Mod Engine path should be simplified");
                Some(simplified.into_os_string().into_string().expect("Os string should have been converted to normal string"))
            }),
        }
    }
}

#[tauri::command]
#[define_cmd]
pub async fn upsert_game_instance(
    game_instance_data_model: GameInstanceDataModel,
    database: State<'_, Database>,
    steam_locate: State<'_, SteamLocate>,
    handle: AppHandle
) -> ManagerResult<GameInstance> {
    const QUERY: &str = r"fn::upsert($id, $data)";
    let data = game_instance_data_model.with_defaults(&steam_locate, &handle);
    let result: Option<_> = database
        .conn
        .query(QUERY)
        .bind(("id", Thing::from(MAIN_INSTANCE)))
        .bind(("data", data))
        .await?
        .take(0)?;
    let Some(instance) = result else { generic!("Failed to create or update the game instance") };

    Ok(instance)
}

#[tauri::command]
#[define_cmd]
pub async fn fetch_the_game_instance(
    database: State<'_, Database>
) -> ManagerResult<GameInstance> {
    Ok(database.get_instance().await?)
}

#[tauri::command]
#[define_cmd]
pub async fn update_game_instance(
    game_instance: GameInstance,
    database: State<'_, Database>,
) -> ManagerResult<GameInstance> {
    let error = anyhow!("Failed to update game instance with id {:#?}", &game_instance.id);
    let result = database
        .conn
        .update(&game_instance)
        .content(Into::<GameInstanceDataModel>::into(game_instance))
        .await?
        .ok_or(error)?;

    Ok(result)
}

#[tauri::command]
#[define_cmd]
pub async fn reset_game_instance(
    database: State<'_, Database>,
    steam_locate: State<'_, SteamLocate>,
    app_handle: AppHandle,
) -> ManagerResult<GameInstance> {
    let instance = database
        .conn
        .update(MAIN_INSTANCE)
        .content(GameInstanceDataModel::default().with_defaults(&steam_locate, &app_handle))
        .await?
        .ok_or(anyhow!("Failed to reset game instance"))?;
    Ok(instance)
}