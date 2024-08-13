use anyhow::anyhow;
use tauri::State;
use macros::define_cmd;
use crate::generic;
use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::schema::{GameInstance, GameInstanceDataModel};

#[tauri::command]
#[define_cmd]
pub async fn create_game_instance(
    game_instance_data_model: GameInstanceDataModel,
    database: State<'_, Database>,
) -> ManagerResult<GameInstance> {
    let result = database
        .conn
        .create("GameInstances")
        .content(game_instance_data_model)
        .await?
        .into_iter()
        .next();

    let Some(instance) = result else { generic!("Failed to create game instance") };
    Ok(instance)
}

#[tauri::command]
#[define_cmd]
pub async fn fetch_all_game_instances(
    database: State<'_, Database>
) -> ManagerResult<Vec<GameInstance>> {
    let result = database
        .conn
        .select("GameInstances")
        .await?;

    Ok(result)
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
        .content(game_instance)
        .await?
        .ok_or(error)?;

    Ok(result)
}

#[tauri::command]
#[define_cmd]
pub async fn reset_game_instance() ->  {
    
    todo!()
}