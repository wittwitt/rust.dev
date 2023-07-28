use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub name: String,
    pub age: i32,
}

impl Default for Setting {
    fn default() -> Self {
        Setting {
            name: String::from("张三"),
            age: 10,
        }
    }
}

impl Setting {
    pub fn new() -> Result<Setting, ConfigError> {
        Config::builder()
            .add_source(File::with_name("cfg.json").required(false))
            .add_source(Environment::with_prefix("HAHA_ENV"))
            .build()?
            .try_deserialize()
    }
}
