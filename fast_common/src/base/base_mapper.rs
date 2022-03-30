use anyhow::Result;
use async_trait::async_trait;
use rbatis::crud::CRUDTable;
use serde::de::DeserializeOwned;

use crate::base::base_model::BaseModel;
use crate::config::orm_config::{RbatisMapper, SerOrmMapper};
