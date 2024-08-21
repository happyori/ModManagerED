// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        .invoke_handler(generate_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}