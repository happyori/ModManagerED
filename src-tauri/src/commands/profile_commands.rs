use anyhow::anyhow;
use tauri::State;
use macros::define_cmd;
use crate::database_id::DbID;
use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::plugins::database_trait::DatabaseTrait;
use crate::schema::{ModInfo, Profile, ProfileDataModel};

#[tauri::command]
#[define_cmd]
pub async fn create_profile(
    profile_data_model: ProfileDataModel,
    database: State<'_, Database>
) -> ManagerResult<Profile> {
    let error = anyhow!("Failed to create profile: {:?}", &profile_data_model.name);
    let result = database
        .conn
        .create("Profiles")
        .content(profile_data_model)
        .await?
        .into_iter()
        .next()
        .ok_or(error)?;

    Ok(result)
}

#[tauri::command]
#[define_cmd]
pub async fn update_profile(
    profile: Profile,
    database: State<'_, Database>
) -> ManagerResult<Profile> {
    let result = database
        .conn
        .update(&profile)
        .content(profile)
        .await?
        .ok_or(anyhow!("Failed to update profile"))?;

    Ok(result)
}

#[tauri::command]
#[define_cmd]
pub async fn delete_profile(
    profile: Profile,
    database: State<'_, Database>
) -> ManagerResult<Option<Profile>> {
    let profile = database
        .conn
        .delete(profile)
        .await?;


    Ok(profile)
}

#[tauri::command]
#[define_cmd]
pub async fn fetch_all_profiles(database: State<'_, Database>) -> ManagerResult<Vec<Profile>> {
    let result = database
        .conn
        .select("Profiles")
        .await?;

    Ok(result)
}

#[tauri::command]
#[define_cmd]
pub async fn enable_mod(profile_id: DbID, mod_id: DbID, database: State<'_, Database>) -> ManagerResult<()> {
    database.enable_mod(profile_id, mod_id).await?;
    Ok(())
}

#[tauri::command]
#[define_cmd]
pub async fn disable_mod(profile_id: DbID, mod_id: DbID, database: State<'_, Database>) -> ManagerResult<()> {
    database.disable_mod(profile_id, mod_id).await?;
    Ok(())
}

#[tauri::command]
#[define_cmd]
pub async fn get_active_mods(profile_id: DbID, database: State<'_, Database>) -> ManagerResult<Vec<ModInfo>> {
    let result = database.get_active_mods(profile_id).await?;
    Ok(result)
}