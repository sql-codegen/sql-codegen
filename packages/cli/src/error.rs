use crate::config;
use postgres;
use sqlparser;
use std::io;

#[derive(Debug)]
pub enum CodegenError {
    ConfigError(config::ConfigError),
    IoError(io::Error),
    PluginNotFoundError(String),
    ParserError(sqlparser::parser::ParserError),
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

impl From<sqlparser::parser::ParserError> for CodegenError {
    fn from(error: sqlparser::parser::ParserError) -> Self {
        CodegenError::ParserError(error)
    }
}

impl From<postgres::Error> for CodegenError {
    fn from(error: postgres::Error) -> Self {
        CodegenError::PostgresError(error)
    }
}
