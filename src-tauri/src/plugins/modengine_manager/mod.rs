use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use subprocess::Exec;
use tauri::PathResolver;

use crate::manager_error::ManagerResult;
use crate::plugins::database::Database;
use crate::plugins::database_trait::DatabaseTrait;
use crate::plugins::modengine_manager::mod_engine_config::{Mod, ModEngineConfig};
use crate::schema::{ModInfo, Profile};

mod mod_engine_config;

#[derive(Debug)]
pub struct ModEngineManager;

impl ModEngineManager {
    pub async fn launch_modded(profile: &Profile, database: &Database, path_resolver: PathResolver) -> ManagerResult<()> {
        let mods = database.get_active_mods(profile.id.clone()).await?;
        let (dll_paths, main) = Self::split_mods(mods);
        let config = ModEngineConfig::new(dll_paths, main);
        let data_file = path_resolver
            .app_local_data_dir()
            .ok_or(anyhow!("Could not get local data folder to store the configuration"))?
            .join(format!("profile_{}.toml", profile.id.id.to_string()));
        let toml = toml::to_string(&config)?;
        fs::write(&data_file, toml)?;
        let launcher_path = Self::get_launcher_path(&path_resolver, database).await;
        let exe = Exec::cmd(launcher_path)
            .args(&["-t", "er"])
            .args(&["-c", data_file.to_str().unwrap()]);
        let _ = exe.communicate();
        Ok(())
    }

    async fn get_launcher_path(path_resolver: &PathResolver, database: &Database) -> PathBuf {
        let game_instance = database.get_instance().await;
        let default_path = path_resolver
            .resolve_resource("modengine/modengine2_launcher.exe");
        if let Ok(game_instance) = game_instance {
            game_instance.mod_engine_path
                .map(|str| PathBuf::from(str).join("modengine2_launcher.exe"))
                .or(default_path)
                .expect("GameInstance or Resources should have modengine launcher")
        } else {
            default_path
                .expect("Mod Engine should exist in resources")
        }
    }

    fn split_mods(mods: Vec<ModInfo>) -> (Vec<String>, Vec<Mod>) {
        let dll_paths = mods
            .iter()
            .filter(|info| info.is_dll)
            .map(|info| &info.path)
            .map(|str| str.replace('\\', r#"\\"#))
            .collect::<Vec<_>>();
        let main: Vec<_> = mods
            .into_iter()
            .filter(|info| !info.is_dll)
            .map(|info| Mod::from(info).enable())
            .collect();

        (dll_paths, main)
    }
}