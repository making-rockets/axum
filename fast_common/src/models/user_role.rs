use rbatis::CRUDTable;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::base::base_model::BaseModel;

#[derive(Debug, Deserialize, Serialize, Clone, CRUDTable)]
pub struct UserRole {
    id: Option<u64>,
    user_name: Option<String>,
    user_id: Option<u64>,
    role_id: Option<u64>,
    role_name: Option<String>,
}

impl BaseModel for UserRole {
    type Model = Self;
}
