extern crate fast_common;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, AddExtensionLayer, Router};
use rbatis::rbatis::Rbatis;

pub mod controller;
mod mapper;
mod routers;
mod service;

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

async fn handler() -> &'static str {
    "hello,world"
}

#[tokio::main]
async fn main() {
    let rb = Rbatis::new();
    // rb.link(MYSQL_URL).await.expect("rbatis link database fail");
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
