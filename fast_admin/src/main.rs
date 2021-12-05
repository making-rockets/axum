extern crate fast_common;

use axum::{routing::get, Router};
use std::net::SocketAddr;

use rbatis::rbatis::Rbatis;
use std::sync::Arc;

use axum::AddExtensionLayer;

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

async fn handler() -> &'static str {
    "hello,world"
}

#[tokio::main]
async fn main() {
    //log
    // fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);

    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .layer(AddExtensionLayer::new(rb));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
