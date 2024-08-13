use serde::{Deserialize, Serialize};
use surrealdb::opt::{IntoResource, Resource};
use macros::{DeriveDataModel, GenerateTypescript};
use crate::database_id::DbID;

macro_rules! impl_into_resource {
    ($srct: ident) => {
        impl From<$srct> for Resource {
            fn from(value: $srct) -> Self {
                value.id.into()
            }
        }
        impl From<&$srct> for Resource {
            fn from(value: &$srct) -> Self {
                value.id.as_ref().into()
            }
        }
        impl IntoResource<Option<$srct>> for $srct {
            fn into_resource(self) -> surrealdb::Result<Resource> {
                Ok(self.id.into())
            }
        }
        impl IntoResource<Option<$srct>> for &$srct {
            fn into_resource(self) -> surrealdb::Result<Resource> {
                Ok(self.id.as_ref().into())
            }
        }
    };
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
    pub deployment_path: Option<String>,
    #[required]
    pub is_dll: bool,
    #[required]
    pub name: String,
    #[required]
    pub path: String,
}

impl_into_resource!(GameInstance);
impl_into_resource!(ModInfo);
impl_into_resource!(Profile);