use anyhow::Result;
use async_trait::async_trait;
use rbatis::crud::CRUDTable;
use serde::de::DeserializeOwned;

use crate::base::base_model::BaseModel;
use crate::config::orm_config::{RbatisMapper, SerOrmMapper};

#[async_trait]
pub trait BaseMapper<Model>
where
    Model: BaseModel + CRUDTable + DeserializeOwned + Sync + Send,
{
    //type M: BaseModel + Sync + Send + Sized + 'static;

    fn save(&self, model: Model) -> Result<()> {
        Ok(())
    }

    fn update(&self, model: Model) -> Result<()> {
        Ok(())
    }

    fn delete(&self, id: String) -> Result<()> {
        Ok(())
    }
    fn list() -> Result<Vec<Box<dyn BaseModel>>> {
        Ok(todo!())
    }

    async fn get_default_mapper() -> RbatisMapper {
        let mapper = RbatisMapper::new().await;
        mapper
    }
}
