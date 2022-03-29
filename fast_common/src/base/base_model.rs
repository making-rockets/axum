use async_trait::async_trait;

#[async_trait]
pub trait BaseModel {
    async fn get_model(&self);
}
