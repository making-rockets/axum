use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiResult<T> {
    code: Option<u32>,
    message: Option<String>,
    data: Option<T>,
}

impl<T> From<T> for ApiResult<T> where T: Serialize + ToString  {
    fn from(t: T) -> Self {
        Self {
            code: Some(200),
            message: None,
            data: Some(t),
        }
    }
}


impl<T> ToString for ApiResult<T> where T: Serialize, {
    fn to_string(&self) -> String {
        let result = serde_json::to_string(&self);
        match result {
            Ok(string) => string,
            Err(err) => panic!("{}，failed to transform string", err.to_string())
        }
    }
}

impl<T> ApiResult<T> where T: DeserializeOwned + Serialize, {
    pub fn new(t: T) -> Result<Self> {
        Ok(Self {
            code: Some(200),
            message: None,
            data: Some(t),
        })
    }

    pub fn error(message: &str) -> Self {
        Self {
            code: Some(500),
            message: Some(message.into()),
            data: None,
        }
    }
    pub fn ok(t: T) -> Self {
        Self {
            code: Some(200),
            message: None,
            data: Some(t),
        }
    }

    pub fn from_result(result: Result<T>) -> Self {
        match result {
            Ok(result) => Self::ok(result),

            Err(err) => Self::error(err.to_string().as_str()),
        }
    }
}
