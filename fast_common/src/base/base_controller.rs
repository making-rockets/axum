use async_trait::async_trait;
use serde::Serialize;

use base_model::BaseModel;

use crate::base::base_model;
use crate::base::base_service::BaseService;
use crate::config::orm_config::Mapper;
use crate::models::user::UserVo;

#[async_trait]
pub trait BaseController<Service, Model>
where
    Service: BaseService<Mapper, Model> + Serialize + Sync + Send,
    Model: BaseModel + Serialize + Sync + Send,
{
    async fn get_user() -> Option<UserVo> {
        None
    }
}
