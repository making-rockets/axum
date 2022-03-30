use anyhow::Result;
use async_trait::async_trait;
use rbatis::crud::CRUDTable;
use serde::de::DeserializeOwned;

use crate::base::base_model::BaseModel;
use crate::config::orm_config::RbatisMapper;

#[async_trait]
pub trait BaseService {
    type Model: BaseModel + CRUDTable + DeserializeOwned + Send;

    async fn save(&self, model: Self::Model) -> Result<()> {
        Ok(())
    }

    fn update(&self, model: Self::Model) -> Result<()> {
        Ok(())
    }

    fn delete(&self, id: String) -> Result<()> {
        Ok(())
    }
    fn list() -> Result<Vec<Box<Self::Model>>> {
        Ok(todo!())
    }

    async fn get_default_mapper() -> RbatisMapper {
        let mapper = RbatisMapper::new().await;
        mapper
    }
}
