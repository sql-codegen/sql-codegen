use crate::args::Args;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path;

const DEFAULT_CONFIG_FILE_NAME: &str = "sql-codegen.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigConnection {
    pub host: String,
    pub user: String,
    pub port: u16,
    pub database: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub connection: ConfigConnection,
    pub schema: String,
    pub queries: String,
}

impl Config {
    pub fn new(args: Args) -> Config {
        let config_file_path = args.config.unwrap_or(Config::find_config_file_path());
        let config_file_content =
            fs::read_to_string(config_file_path).expect("Error reading config file");
        serde_json::from_str(&config_file_content).expect("Error parsing config file")
    }

    fn find_config_file_path() -> String {
        let mut current_dir = path::PathBuf::from(env::current_dir().unwrap());
        loop {
            let config_file_path = current_dir.join(DEFAULT_CONFIG_FILE_NAME);
            if config_file_path.exists() {
                return config_file_path.to_str().unwrap().to_string();
            }
            current_dir.pop();
            if current_dir.to_str() == Some("/") {
                panic!("No config file found");
            }
        }
    }
}
