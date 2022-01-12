use crate::error::CodegenError;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path;

const DEFAULT_CONFIG_FILE_NAME: &str = "sql-codegen.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionConfig {
    pub host: String,
    pub user: String,
    pub port: u16,
    pub database: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateConfig {
    pub output: String,
    pub plugins: Vec<PluginConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginConfig {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub dialect: String,
    pub connection: ConnectionConfig,
    pub schema: String,
    pub queries: String,
    pub generate: Vec<GenerateConfig>,
}

impl Config {
    pub fn new(config_file_path: &Option<String>) -> Result<Config, CodegenError> {
        let config_file_path = match &config_file_path {
            Some(config_file_path) => Some(config_file_path.clone()),
            None => Config::find_config_file_path(),
        };
        if config_file_path.is_none() {
            return Err(CodegenError::ConfigError(
                "The config file not found in current or parent directories.".to_string(),
            ));
        }
        let config_file_content =
            fs::read_to_string(config_file_path.unwrap()).or_else(|error| {
                if let ErrorKind::NotFound = error.kind() {
                    return Err(CodegenError::ConfigError(
                        "The config file does not exist.".to_string(),
                    ));
                } else {
                    return Err(CodegenError::ConfigError(
                        "Error reading config file.".to_string(),
                    ));
                }
            })?;
        let config = serde_json::from_str::<Config>(&config_file_content).or_else(|_| {
            Err(CodegenError::ConfigError(
                "Error parsing config file.".to_string(),
            ))
        })?;
        Ok(config)
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
