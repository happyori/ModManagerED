use std::path::PathBuf;
use std::sync::Mutex;

use steamlocate::SteamDir;
use tracing::debug;

#[derive(Debug)]
pub struct SteamLocate {
    steam_dir: Mutex<SteamDir>,
}

const ELDEN_RING_ID: u32 = 1245620;

impl SteamLocate {
    pub fn new(steam_dir: SteamDir) -> Self {
        Self {
            steam_dir: Mutex::new(steam_dir),
        }
    }

    pub fn init() -> anyhow::Result<Self> {
        let dir = SteamDir::locate()?;
        Ok(Self::new(dir))
    }

    pub fn get_elden_ring_install(&self) -> Option<PathBuf> {
        debug!("Attempting to locate elden ring install");
        self.steam_dir
            .lock()
            .unwrap()
            .find_app(ELDEN_RING_ID)
            .ok()?
            .map(|(app, lib)| lib.resolve_app_dir(&app))
    }

    pub fn get_elden_ring_install_str(&self) -> Option<String> {
        self.get_elden_ring_install()
            .map(|path| path.into_os_string())
            .map(|os| os.into_string().unwrap())
    }
}