use crate::schema::ModInfo;
use async_trait::async_trait;
use crate::database_id::DbID;

#[async_trait]
pub trait DatabaseTrait {
    async fn get_active_mods(&self, profile: DbID) -> anyhow::Result<Vec<ModInfo>>;
    async fn enable_mod(&self, profile: DbID, mod_info: DbID) -> anyhow::Result<()>;
    async fn disable_mod(&self, profile: DbID, mod_info: DbID) -> anyhow::Result<()>;
}