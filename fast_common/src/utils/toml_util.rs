use std::path::Path;

use sea_orm::sea_query::any;
use toml_edit::{Document, Item};

pub struct ParseToml {}

impl ParseToml {
    pub async fn parse_toml<'a>(toml: &Path, type_str: &str) -> anyhow::Result<Item> {
        let file_str = std::fs::read_to_string(toml)?;
        let document = file_str.parse::<Document>()?;
        let item = document[type_str].to_owned();
        Ok(item)
    }

    pub async fn parse_tome_of_setting_toml() -> anyhow::Result<Item> {
        ParseToml::parse_toml(Path::new("../setting.toml"), "database").await
    }
}

#[test]
pub fn test() {
    ParseToml::parse_toml(Path::new("../setting.toml"), "database");
}
