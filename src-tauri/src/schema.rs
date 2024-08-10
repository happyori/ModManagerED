use serde::{Deserialize, Serialize, Serializer};
use surrealdb::sql::Thing;

use macros::{DeriveDataModel, GenerateTypescript};

#[derive(Debug, Deserialize, Clone)]
pub struct DbID(Thing);

impl Serialize for DbID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.0.to_raw())
    }
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone, GenerateTypescript)]
#[gen(directory = "../src/generated")]
pub struct GameInstance {
    #[omitted]
    #[gen(typed_as = String)]
    pub id: DbID,
    #[required]
    pub path: String,
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone, GenerateTypescript)]
#[gen(directory = "../src/generated")]
pub struct Profile {
    #[gen(typed_as = String)]
    #[omitted]
    pub id: DbID,
    #[required]
    pub name: String,
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone, GenerateTypescript)]
#[gen(directory = "../src/generated")]
pub struct ModInfo {
    #[gen(typed_as = String)]
    #[omitted]
    pub id: DbID,
    #[optional]
    pub deployment_path: String,
    #[required]
    pub is_dll: bool,
    #[required]
    pub name: String,
    #[required]
    pub path: String,
}