// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::Arc;

use anyhow::anyhow;
use figment::{
    Figment,
    providers::{Format, Yaml}
};
use tauri::Manager;
use taurpc::Router;
use window_shadows::set_shadow;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use window_vibrancy::apply_mica;

use crate::commands::{
    GameInstanceApi, GameInstanceApiImpl,
    ModInfoApi, ModInfoApiImpl,
    ModManagementApi, ModManagementApiImpl,
    ProfileApi, ProfileApiImpl,
};
use crate::plugins::database::Database;
use crate::plugins::steamlocate::SteamLocate;

mod plugins;
mod schema;
mod commands;
pub mod manager_error;
mod database_id;

#[cfg(debug_assertions)]
const ENV: &str = "dev";
#[cfg(not(debug_assertions))]
const ENV: &str = "release";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let figment = Figment::new()
        .merge(Yaml::file("database_settings.yml").nested())
        .select(ENV);

    let database = Arc::new(Database::new(figment.extract()?).await?);
    let steam_locate = Arc::new(SteamLocate::init()?);
    let router = Router::new()
        .merge(ModInfoApiImpl::new(database.clone()).into_handler())
        .merge(ProfileApiImpl::new(database.clone()).into_handler())
        .merge(ModManagementApiImpl::new(database.clone()).into_handler())
        .merge(GameInstanceApiImpl::new(database.clone(), steam_locate.clone()).into_handler());

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            for window in app.windows().values() {
                #[cfg(any(windows, target_os = "macos"))]
                set_shadow(window, true).map_err(|e| anyhow!(e.to_string()))?;
                #[cfg(target_os = "windows")]
                apply_mica(window, None)?;
                #[cfg(target_os = "macos")]
                apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)?;
            }
            Ok(())
        })
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}