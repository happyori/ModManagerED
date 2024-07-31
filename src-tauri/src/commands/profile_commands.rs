use anyhow::anyhow;
use tauri::State;
use macros::define_cmd;
use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::schema::{Profile, ProfileDataModel};

#[tauri::command]
#[define_cmd]
pub async fn create_profile(profile_data_model: ProfileDataModel, database: State<'_, Database>) -> ManagerResult<Profile> {
    let result = database
        .conn
        .create("Profiles")
        .content(profile_data_model.clone())
        .await?
        .into_iter()
        .next()
        .ok_or(anyhow!("Failed to create profile: {:?}", &profile_data_model.name))?;

    Ok(result)
}