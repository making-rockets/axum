use crate::service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, Json};
use crate::db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_user_online::{SysUserOnlineDeleteReq, SysUserOnlineSearchReq},
        prelude::SysUserOnlineModel,
    },
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysUserOnlineSearchReq>) -> Res<ListData<SysUserOnlineModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 删除
pub async fn delete(Json(delete_req): Json<SysUserOnlineDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::delete(db, delete_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 登出
pub async fn log_out(user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::log_out(db, user.token_id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
