use axum::{
    http::StatusCode,
    middleware,
    routing::{get, get_service,},
    Router,
};
use tower_http::services::ServeDir;
use crate::config::CFG;
use crate::middleware::{ApiAuth, Cache, Ctx, OperLog, SkyTableCache};
use crate::service::service_utils::jwt::Claims;
use crate::service::system::sys_user::login;
use crate::system::log_out;

use super::{system, test};

pub fn api() -> Router {
    Router::new()
        // 文件上传api
        .nest_service(
            &CFG.web.upload_url,
            get_service(ServeDir::new(&CFG.web.upload_dir))
                .handle_error(|error: std::io::Error| async move { (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error)) }),
        )
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        // 系统管理模块
        .nest("/system", set_auth_middleware(system::system_api()))
        //  测试模块
        .nest("/test", test_api())
}

// 无需授权api
fn no_auth_api() -> Router {
    Router::new()
        //.route("/login", post(login)) // 登录
        .route("/get_captcha", get(system::get_captcha)) // 获取验证码
        //.route("/log_out", post( log_out)) // 退出登录
}

// 设置授权路由的中间件
fn set_auth_middleware(router: Router) -> Router {
    let router = match &CFG.log.enable_oper_log {
        true => router.layer(middleware::from_fn(OperLog)),
        false => router,
    };
    let router = match CFG.server.cache_time {
        0 => router,
        _ => {
            if CFG.server.cache_method == 0 {
                router.layer(middleware::from_fn( Cache))
            } else {
                router.layer(middleware::from_fn( SkyTableCache))
            }
        }
    };
    #[allow(clippy::let_and_return)]
    let router = router
        .layer(middleware::from_fn(ApiAuth))
        .layer(middleware::from_fn( Ctx))
        .layer(middleware::from_extractor::<Claims>());
    router
}

// 测试api
pub fn test_api() -> Router {
    let router = test::test_api();

    let router = match &CFG.log.enable_oper_log {
        true => router.layer(middleware::from_fn( OperLog)),
        false => router,
    };
    router
        .route_layer(middleware::from_fn(ApiAuth))
        .layer(middleware::from_fn( Ctx))
        .layer(middleware::from_extractor::<Claims>())
}
