use async_trait::async_trait;
use rbatis::crud::CRUDTable;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait BaseModel {
    type Model: CRUDTable + Serialize + Send + Sync + DeserializeOwned;
}
