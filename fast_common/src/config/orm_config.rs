use std::ops::{Deref, DerefMut};
use std::time::Duration;

use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;

use rbatis::db::DBPoolOptions;
use rbatis::rbatis::Rbatis;

use crate::utils::toml_util;

lazy_static! {
    static ref rbatis: Rbatis = Rbatis::new();
}

pub struct RbatisMapper {
    rbatis: rbatis::rbatis::Rbatis,
}

impl Deref for RbatisMapper {
    type Target = Rbatis;

    fn deref(&self) -> &Self::Target {
        &Rbatis
    }
}

impl DerefMut for RbatisMapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut Rbatis
    }
}

pub struct SerOrmMapper {
    ser_orm: DatabaseConnection,
}

impl Mapper {}

impl RbatisMapper {
    pub async fn new() -> Self {
        use rbatis::crud::CRUD;
        use rbatis::db::DBPoolOptions;
        use rbatis::rbatis::Rbatis;
        let rb = Rbatis::new();
        ///连接数据库,自动判断驱动类型"mysql://*","postgres://*","sqlite://*","mssql://*"加载驱动   
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();

        let mut opt = DBPoolOptions::new();
        opt.max_connections = 100;
        rb.link_opt("mysql://root:root@39.101.69.31:3306/test", opt)
            .await
            .unwrap();
        Self { rbatis: rb }

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
