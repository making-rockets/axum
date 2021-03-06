use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

use crate::models::menu::MenuVo;
use crate::models::user_role::UserRole;
use crate::rbatis::CRUDTable;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, CRUDTable)]
pub struct User {
    pub id: Option<u64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserVo {
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub age: Option<u64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub create_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserLoginVo {
    pub token: Option<String>,
    pub user_name: Option<String>,
    pub user_id: Option<u64>,
    pub password: Option<String>,
    pub bar_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserRoleMenuVo {
    pub user_id: Option<u64>,
    pub user_name: Option<String>,
    pub access_token: Option<String>,
    pub role_id: Option<u64>,
    pub role_name: Option<String>,
    pub menus: Option<Vec<MenuVo>>,
}
