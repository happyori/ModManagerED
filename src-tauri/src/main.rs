// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::create_dir_all;
use std::sync::Arc;

use anyhow::anyhow;
use figment::{
    Figment,
    providers::{Format, Yaml}
};
use tauri::{Assets, Config, Context, Manager};
use tauri::api::path::app_log_dir;
use taurpc::Router;
use tracing::{debug, error, info, instrument, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use window_shadows::set_shadow;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use window_vibrancy::{apply_mica};

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

#[instrument(skip(context))]
async fn run<A: Assets>(context: Context<A>) -> anyhow::Result<()> {
    debug!("Settings up database config with profile: {ENV}");
    let figment = Figment::new()
        .merge(Yaml::file("database_settings.yml").nested())
        .select(ENV);
    debug!("Setup figment, \n {:#?}", &figment);

    debug!("Initializing Database");
    let database = Arc::new(Database::new(figment.extract()?).await?);
    info!("Database initialized");
    debug!("Initializing Steam Locate");
    let steam_locate = Arc::new(SteamLocate::init()?);
    info!("Steam Locate Initialized");
    let router = Router::new()
        .merge(ModInfoApiImpl::new(database.clone()).into_handler())
        .merge(ProfileApiImpl::new(database.clone()).into_handler())
        .merge(ModManagementApiImpl::new(database.clone()).into_handler())
        .merge(GameInstanceApiImpl::new(database.clone(), steam_locate.clone()).into_handler());

    info!("Starting the app!");
    tauri::Builder::default()
        .setup(|app| {
            info!("Setting up application decorations");
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            for window in app.windows().values() {
                #[cfg(any(windows, target_os = "macos"))]
                set_shadow(window, true).map_err(|e| anyhow!(e.to_string()))?;
                #[cfg(target_os = "windows")]
                apply_mica(window, Some(true))?;
                #[cfg(target_os = "macos")]
                apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)?;
            }
            Ok(())
        })
        .invoke_handler(router.into_handler())
        .run(context)?;
    info!("Finished, exiting");

    Ok(())
}

fn setup_logging(config: &Config) -> anyhow::Result<WorkerGuard> {
    let log_dir = app_log_dir(config).ok_or(anyhow!("Failed to get log directory!"))?;
    create_dir_all(&log_dir)?;
    let file_appender = tracing_appender::rolling::hourly(log_dir, "golden_order.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_target(true)
        .with_file(false)
        .with_line_number(false);

    let debug_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(
            stdout_layer
                .with_filter(filter::LevelFilter::INFO)
                .and_then(debug_log)
                .with_filter(filter::LevelFilter::TRACE)
                .with_filter(filter::filter_fn(|metadata| {
                    let is_info = metadata.level() == &Level::INFO;
                    let from_crate = metadata
                        .module_path()
                        .map(|path| path.contains("goldenorder"))
                        .unwrap_or(false);
                    is_info || from_crate
                }))
        )
        .init();

    Ok(guard)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let context = tauri::generate_context!();
    let _guard = setup_logging(context.config())?;
    match run(context).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error! {
                %e,
                "Something went wrong!"
            }
            Ok(())
        }
    }
}