use crate::config;
use postgres;
use std::io;

#[derive(Debug)]
pub enum CodegenError {
    ConfigError(config::ConfigError),
    IoError(io::Error),
    PostgresError(postgres::Error),
}

impl From<config::ConfigError> for CodegenError {
    fn from(error: config::ConfigError) -> Self {
        CodegenError::ConfigError(error)
    }
}

impl From<io::Error> for CodegenError {
    fn from(error: io::Error) -> Self {
        CodegenError::IoError(error)
    }
}

impl From<postgres::Error> for CodegenError {
    fn from(error: postgres::Error) -> Self {
        CodegenError::PostgresError(error)
    }
}
