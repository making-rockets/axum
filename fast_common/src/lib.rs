#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

pub mod base;
pub mod common;
mod config;
pub mod middleware;
pub mod models;
pub mod service;
pub mod utils;

lazy_static! {
    static ref RB: rbatis::rbatis::Rbatis = rbatis::rbatis::Rbatis::new();
}
