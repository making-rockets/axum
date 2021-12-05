use async_trait::async_trait;
use serde::Serialize;
use std::future::Future;

#[async_trait]
pub trait BaseStorage: Sync + Send {
    async fn cache_entity<'a, T>(&self, key: &'a String, t: &'a T) -> Result<String, &'a str>
    where
        &'a T: Send + Serialize + Sync;
    async fn get_entity<T>(&self, key: &String)
    where
        T: Send + Serialize + Sync;
}
