use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Menu {
    pub id: Option<u64>,
    pub menu_name: Option<String>,
    pub parent_id: Option<u64>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub index_no: Option<i32>,
    pub remark: Option<String>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MenuVo {
    pub id: Option<u64>,
    pub menu_name: Option<String>,
    pub parent_id: Option<u64>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub index_no: Option<i32>,
    pub remark: Option<String>,
    pub state: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
