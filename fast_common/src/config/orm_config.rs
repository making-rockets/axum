use std::ops::{Deref, DerefMut};
use std::time::Duration;

use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use rbatis::crud::CRUD;
use rbatis::db::DBPoolOptions;
use rbatis::intercept::SqlIntercept;
use rbatis::log::LogPlugin;
use rbatis::rbatis::Rbatis;
use sea_orm::DatabaseConnection;

use crate::utils::toml_util;

pub struct RbatisMapper {
    rbatis: rbatis::rbatis::Rbatis,
}

pub struct SerOrmMapper {
    ser_orm: DatabaseConnection,
}

impl RbatisMapper {
    pub async fn new() -> Self {
        let mut rbatis = Rbatis::new();
        rbatis.add_sql_intercept(Intercept {});
        rbatis.set_log_plugin(RbatisLog {});
        rbatis
            .link_opt("mysql://root:root@39.101.69.31:3306/test", db_option())
            .await
            .unwrap();
        println!("rbatis init success");
        Self { rbatis }

        //启用日志输出，你也可以使用其他日志框架，这个不限定的
        //fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    }
}

impl SerOrmMapper {
    pub async fn new() -> Self {
        use sea_orm::{ConnectOptions, Database};
        let mut opt =
            sea_orm::ConnectOptions::new("mysql://root:root@39.101.69.31:3306/test".to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let db = Database::connect(opt).await.unwrap();
        Self { ser_orm: db }
    }
}

#[derive(Debug)]
pub struct Intercept {}

impl SqlIntercept for Intercept {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn do_intercept(
        &self,
        _rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<bson2::Bson>,
        _is_prepared_sql: bool,
    ) -> Result<(), rbatis::core::Error> {
        debug!(target: "app_events", "sql = : x: {:?}, y: {:?}",sql, args);
        Ok(())
    }
}

#[derive(Debug)]
pub struct RbatisLog {}

impl LogPlugin for RbatisLog {
    fn get_level_filter(&self) -> &LevelFilter {
        &LevelFilter::Debug
    }
}

pub fn db_option() -> DBPoolOptions {
    let mut opt = DBPoolOptions::new();
    opt.min_connections = 1;
    opt.max_connections = 100;
    opt.connect_timeout = Duration::from_secs(100);
    opt.max_lifetime = Some(Duration::from_secs(1800));
    opt.test_before_acquire = true;
    return opt;
}
