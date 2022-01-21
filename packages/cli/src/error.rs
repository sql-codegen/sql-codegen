// use crate::duplicated_identifier::DuplicatedIdentifierError;
use postgres;
use sqlparser;
use std::io;

#[derive(Debug)]
pub enum CodegenError {
    ConfigError(String),
    // DuplicatedIdentifierError(DuplicatedIdentifierError),
    IoError(io::Error),
    PluginNotFoundError(String),
    ParserError(sqlparser::parser::ParserError),
    PostgresError(postgres::Error),
    QueryError(String),
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
