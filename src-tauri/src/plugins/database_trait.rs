use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::opt::IntoResource;

use crate::manager_error::ManagerResult;

#[async_trait]
pub trait DatabaseTrait {
    async fn create<R, RModel>(&self, resource: impl IntoResource<Vec<R>> + Send, content: RModel) -> ManagerResult<R>
    where
        R: Send + DeserializeOwned,
        RModel: Serialize + Send;
    async fn update<R, RModel>(&self, content: R) -> ManagerResult<Option<R>>
    where
        RModel: Send + Serialize + From<R>,
        R: Send + DeserializeOwned + IntoRefResource;
    async fn delete<R>(&self, content: R) -> ManagerResult<Option<R>>
    where
        R: Send + IntoResource<Option<R>> + DeserializeOwned;

    async fn fetch<R>(&self, resource: impl IntoResource<Vec<R>> + Send) -> ManagerResult<Vec<R>>
    where
        R: Send + DeserializeOwned;
}

pub trait IntoRefResource
where
    Self: Sized,
{
    fn into_referenced_resource(&self) -> impl IntoResource<Option<Self>>;
}