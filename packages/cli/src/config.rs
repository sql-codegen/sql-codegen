use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path;

const DEFAULT_CONFIG_FILE_NAME: &str = "sql-codegen.json";

#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

impl ConfigError {
    fn new(message: &str) -> ConfigError {
        ConfigError {
            message: message.to_string(),
        }
    }
}
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
    pub fn new(config_file_path: &Option<String>) -> Result<Config, ConfigError> {
        let config_file_path = match &config_file_path {
            Some(config_file_path) => Some(config_file_path.clone()),
            None => Config::find_config_file_path(),
        };
        if config_file_path.is_none() {
            return Err(ConfigError::new(
                "The config file not found in current or parent directories.",
            ));
        }
        let config_file_content =
            fs::read_to_string(config_file_path.unwrap()).or_else(|error| {
                if let ErrorKind::NotFound = error.kind() {
                    return Err(ConfigError::new("The config file does not exist."));
                } else {
                    return Err(ConfigError::new("Error reading config file."));
                }
            })?;
        let config = serde_json::from_str::<Config>(&config_file_content)
            .or_else(|_| Err(ConfigError::new("Error parsing config file.")))?;
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
