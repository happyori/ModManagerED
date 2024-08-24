#![allow(dead_code)]

use tauri::AppHandle;
use taurpc::specta;

use crate::database_id::DbID;
use crate::manager_error::{ManagerResult, ModManageredError};
use crate::plugins::database_trait::DatabaseTrait;
use crate::schema::{GameInstance, GameInstanceDataModel, ModInfo, ModInfoDataModel, Profile, ProfileDataModel};

mod mod_management;
mod game_instance;
pub use mod_management::ModManagementApiImpl;
pub use game_instance::GameInstanceApiImpl;

macro_rules! create_api {
    (for $typ:ident with $data:ident, $layer:literal as $api_name:ident) => {
        #[taurpc::procedures(export_to = "../src/generated/binding.ts", path = $layer)]
        pub trait $api_name {
            async fn create(data: $data) -> Result<$typ, ModManageredError>;
            async fn update(data: $typ) -> Result<Option<$typ>, ModManageredError>;
            async fn delete(data: $typ) -> Result<Option<$typ>, ModManageredError>;
            async fn fetch_all() -> Result<Vec<$typ>, ModManageredError>;
        }
    };
}
macro_rules! create_api_impl {
    (impl $api:ident for $name:ident, ($tp:path, $dm:path), as $tbl: literal) => {
        #[derive(Clone)]
        pub struct $name {
            database: std::sync::Arc<crate::plugins::database::Database>
        }

        impl $name {
            pub fn new(database: std::sync::Arc<crate::plugins::database::Database>) -> Self {
                Self { database }
            }
        }

        #[taurpc::resolvers]
        impl $api for $name {
            async fn create(self, data: $dm) -> ManagerResult<$tp> {
                self.database.create::<$tp, $dm>($tbl, data).await
            }

            async fn update(self, data: $tp) -> ManagerResult<Option<$tp>> {
                self.database.update::<$tp, $dm>(data).await
            }

            async fn delete(self, data: $tp) -> ManagerResult<Option<$tp>> {
                self.database.delete(data).await
            }

            async fn fetch_all(self) -> ManagerResult<Vec<$tp>> {
                self.database.fetch($tbl).await
            }
        }
    };
}

create_api!(for ModInfo with ModInfoDataModel, "api.mod" as ModInfoApi);
create_api!(for Profile with ProfileDataModel, "api.profile" as ProfileApi);
create_api_impl!(impl ModInfoApi for ModInfoApiImpl, (ModInfo, ModInfoDataModel), as "ModInfos");
create_api_impl!(impl ProfileApi for ProfileApiImpl, (Profile, ProfileDataModel), as "Profiles");

#[taurpc::procedures(path = "api.mod.manage", export_to = "../src/generated/binding.ts")]
pub trait ModManagementApi {
    async fn enable(profile_id: DbID, mod_id: DbID) -> Result<(), ModManageredError>;
    async fn disable(profile_id: DbID, mod_id: DbID) -> Result<(), ModManageredError>;
    async fn fetch_active(profile_id: DbID) -> Result<Vec<ModInfo>, ModManageredError>;
    async fn launch_game(profile: Profile, app_handle: AppHandle) -> Result<(), ModManageredError>;
}

#[taurpc::procedures(path = "api.game", export_to = "../src/generated/binding.ts")]
pub trait GameInstanceApi {
    async fn upsert(data: GameInstanceDataModel, app_handle: AppHandle) -> Result<GameInstance, ModManageredError>;
    async fn fetch() -> Result<GameInstance, ModManageredError>;
    async fn update(data: GameInstance) -> Result<GameInstance, ModManageredError>;
    async fn reset(app_handle: AppHandle) -> Result<GameInstance, ModManageredError>;
}