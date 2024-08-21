use serde::{Deserialize, Serialize};

use crate::schema::ModInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModEngineConfig {
    pub modengine: ModEngine,
    pub extension: Extension,
}

impl ModEngineConfig {
    pub fn new(external_dlls: Vec<String>, mods: Vec<Mod>) -> Self {
        Self {
            modengine: ModEngine {
                debug: false,
                external_dlls,
            },
            extension: Extension {
                mod_loader: ModLoader {
                    enabled: true,
                    loose_params: false,
                    mods,
                },
                scylla_hide: ScyllaHide { enabled: false },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModEngine {
    pub debug: bool,
    pub external_dlls: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Extension {
    pub mod_loader: ModLoader,
    pub scylla_hide: ScyllaHide,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModLoader {
    pub enabled: bool,
    pub loose_params: bool,
    pub mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mod {
    pub enabled: bool,
    pub name: String,
    pub path: String,
}

impl From<ModInfo> for Mod {
    fn from(value: ModInfo) -> Self {
        Self {
            enabled: false,
            name: value.name,
            path: value.path,
        }
    }
}

impl Mod {
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct ScyllaHide {
    pub enabled: bool,
}