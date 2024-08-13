use anyhow::anyhow;
use tauri::State;
use macros::define_cmd;
use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::schema::{ModInfo, ModInfoDataModel};

#[tauri::command]
#[define_cmd]
pub async fn create_mod_info(
    data: ModInfoDataModel,
    database: State<'_, Database>
) -> ManagerResult<ModInfo> {
    let result = database
        .conn
        .create("ModInfos")
        .content(data)
        .await?
        .into_iter()
        .next()
        .ok_or(anyhow!("Failed to create the mod info"))?;

    Ok(result)
}

#[tauri::command]
#[define_cmd]
pub async fn update_mod_info(
    mod_info: ModInfo,
    database: State<'_, Database>
) -> ManagerResult<Option<ModInfo>> {
    let info = database
        .conn
        .update(&mod_info)
        .content(mod_info)
        .await?;
    Ok(info)
}

#[tauri::command]
#[define_cmd]
pub async fn delete_mod_info(
    mod_info: ModInfo,
    database: State<'_, Database>
) -> ManagerResult<Option<ModInfo>> {
    let info = database
        .conn
        .delete(mod_info)
        .await?;
    Ok(info)
}

#[tauri::command]
#[define_cmd]
pub async fn fetch_all_mods(database: State<'_, Database>) -> ManagerResult<Vec<ModInfo>> {
    let result = database
        .conn
        .select("ModInfos")
        .await?;

    Ok(result)
}