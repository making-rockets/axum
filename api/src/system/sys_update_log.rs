use crate::service::{service_utils::jwt::Claims, system};
use axum::Json;
use crate::db::{
    common::res::Res,
    db_conn,
    system::{
        models::sys_update_log::{SysUpdateLogAddReq, SysUpdateLogDeleteReq, SysUpdateLogEditReq},
        prelude::SysUpdateLogModel,
    },
    DB,
};

/// add 添加
pub async fn add(user: Claims, Json(req): Json<SysUpdateLogAddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::add(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
pub async fn delete(Json(req): Json<SysUpdateLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::soft_delete(db, &req.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
pub async fn edit(user: Claims, Json(req): Json<SysUpdateLogEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::edit(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部
pub async fn get_all() -> Res<Vec<SysUpdateLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
