extern crate confy;

use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub app_name: String,
    pub port: u16,
    pub uri: String,
    pub db: String,
}

impl ::std::default::Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            app_name: "getting-started".to_string(),
            port: 8080,
            uri: "john".to_string(),
            db: "rust".to_string(),
        }
    }
}

impl ApplicationConfig {
    pub fn load_config() -> Result<ApplicationConfig, env::VarError> {
        let key = "CONFIG_PATH";
        match env::var(key) {
            Ok(val) => {
                match confy::load(&val) {
                    Ok(val) => Ok(val),
                    Err(e) => panic!("{}", e)
                }
            }
            Err(e) => panic!("{}", e)
        }
    }
}
