use serde::{Deserialize, Serialize};
use surrealdb::opt::{IntoResource, Resource};
use specta::Type;
use macros::{DeriveDataModel};

use crate::database_id::DbID;
use crate::plugins::database_trait::IntoRefResource;

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
        impl IntoRefResource for $srct {
            fn into_referenced_resource(&self) -> impl IntoResource<Option<Self>> {
                self
            }
        }
    };
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone, Type)]
pub struct GameInstance {
    #[omitted]
    pub id: DbID,
    #[optional]
    pub path: String,
    pub mod_engine_path: Option<String>
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone, Type)]
pub struct Profile {
    #[omitted]
    pub id: DbID,
    #[required]
    pub name: String,
}

#[derive(Serialize, Debug, Deserialize, DeriveDataModel, Clone, Type)]
pub struct ModInfo {
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