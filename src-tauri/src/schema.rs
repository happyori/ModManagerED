use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use macros::{DeriveDataModel, generate_typescript};

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone)]
#[generate_typescript(directory = "../src/types/generated")]
pub struct GameInstance {
    #[omitted]
    #[gen(typed_as = String)]
    pub id: Thing,
    #[required] pub path: String,
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone)]
#[generate_typescript(directory = "../src/types/generated")]
pub struct Profile {
    #[gen(typed_as = String)]
    #[omitted] pub id: Thing,
    #[required] pub name: String,
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone)]
#[generate_typescript(directory = "../src/types/generated")]
pub struct ModInfo {
    #[gen(typed_as = String)]
    #[omitted] pub id: Thing,
    #[optional] pub deployment_path: String,
    #[required] pub is_dll: bool,
    #[required] pub name: String,
    #[required] pub path: String,
}