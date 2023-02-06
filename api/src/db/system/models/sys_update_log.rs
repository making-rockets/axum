use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct SysUpdateLogAddReq {
    pub version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SysUpdateLogEditReq {
    pub id: String,
    pub version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct SysUpdateLogDeleteReq {
    pub id: String,
}
