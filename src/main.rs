use std::env;

use crate::routing::run;

mod config;
mod model;
mod routing;
mod handler;

#[tokio::main]
async fn main() {
    let config_path = "D:\\Work\\Rust-project\\getting-started\\Settings";
    let key = "CONFIG_PATH";
    env::set_var(key, config_path);
    assert_eq!(env::var(key), Ok(config_path.to_string()));
    match config::ApplicationConfig::load_config() {
        Ok(cfg) => {
            let mongo = model::database::connect(cfg.app_name.clone(), cfg.uri.clone(), cfg.db.clone()).await;
            // for db_name in client.list_database_names(None, None).await.expect("Error listing") {
            //     println!("{}", db_name);
            // }
            run(mongo, &cfg.port).await;
        }
        Err(error) => panic!("Problem loading config: {:?}", error),
    };
}
