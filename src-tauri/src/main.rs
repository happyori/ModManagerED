// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macros::generate_commands;
use crate::commands::*;

mod plugins;
mod schema;
mod commands;
pub mod manager_error;
mod database_id;

fn main() {
    tauri::Builder::default()
        .plugin(plugins::database::init())
        .invoke_handler(generate_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}