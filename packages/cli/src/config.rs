use crate::args::Args;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path;

const DEFAULT_CONFIG_FILE_NAME: &str = "sql-codegen.json";

#[derive(Debug)]
pub struct Error<'a> {
    pub message: &'a str,
}

impl<'a> Error<'a> {
    fn new(message: &str) -> Error {
        Error { message }
    }
}

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
    pub fn new(args: Args) -> Result<Config, Error<'static>> {
        let config_file_path = args.config.or_else(|| Config::find_config_file_path());
        if config_file_path.is_none() {
            return Err(Error::new(
                "The config file not found in current or parent directories.",
            ));
        }
        let config_file_content = fs::read_to_string(config_file_path.unwrap()).or_else(|error| {
            if let ErrorKind::NotFound = error.kind() {
                return Err(Error::new("The config file does not exist."));
            } else {
                return Err(Error::new("Error reading config file."));
            }
        });
        let config: Result<Config, Error> = serde_json::from_str(&config_file_content?)
            .or_else(|_| Err(Error::new("Error parsing config file.")));
        Ok(config?)
    }

    fn find_config_file_path() -> Option<String> {
        let mut current_dir = path::PathBuf::from(env::current_dir().unwrap());
        loop {
            let config_file_path = current_dir.join(DEFAULT_CONFIG_FILE_NAME);
            if config_file_path.exists() {
                return Some(config_file_path.to_str().unwrap().to_string());
            }
            current_dir.pop();
            if current_dir.to_str() == Some("/") {
                return None;
            }
        }
    }
}
