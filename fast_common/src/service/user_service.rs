use crate::base::base_model::BaseModel;
use crate::base::base_service::BaseService;
use crate::config::orm_config::RbatisMapper;
use crate::models::user::User;

pub struct UserService {}

impl BaseModel for User {
    type Model = Self;
}

impl BaseService for UserService {
    type Model = User;
    // /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
}
