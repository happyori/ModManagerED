// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::anyhow;
use tauri::Manager;
use window_shadows::set_shadow;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use window_vibrancy::apply_mica;

use macros::generate_commands;

#[allow(unused_imports)]
use crate::commands::*;

mod plugins;
mod schema;
mod commands;
pub mod manager_error;
mod database_id;

fn main() {
    tauri::Builder::default()
        .plugin(plugins::database::init())
        .plugin(plugins::steamlocate::init())
        .setup(|app| {
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
        .invoke_handler(generate_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}