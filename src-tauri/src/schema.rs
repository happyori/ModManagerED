use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
#[derive(Serialize, Debug, Deserialize)]
pub struct GameInstance {
    pub id: Thing,
    pub path: String
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Profile {
    pub id: Thing,
    pub name: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ModInfo {
    pub id: Thing,
    pub deployment_path: String,
    pub is_dll: bool,
    pub name: String,
    pub path: String,
}