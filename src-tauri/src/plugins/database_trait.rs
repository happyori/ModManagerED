use crate::schema::{ModInfo, Profile};
use async_trait::async_trait;

#[async_trait]
pub trait DatabaseTrait {
    async fn get_active_mods(&self, profile: &Profile) -> anyhow::Result<Vec<ModInfo>>;
    async fn enable_mod(&self, profile: &Profile, mod_info: &ModInfo) -> anyhow::Result<()>;
    async fn disable_mod(&self, profile: &Profile, mod_info: &ModInfo) -> anyhow::Result<()>;
}