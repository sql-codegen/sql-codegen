use postgres;
use sqlparser;
use std::io;

#[derive(Debug)]
pub enum CodegenError {
    CliError(String),
    ConfigError(String),
    IoError(io::Error),
    ParserError(sqlparser::parser::ParserError),
    PostgresError(postgres::Error),
    SchemaError(String),
    QueryError(String),
}

impl From<io::Error> for CodegenError {
    fn from(error: io::Error) -> CodegenError {
        CodegenError::IoError(error)
    }
}

impl From<sqlparser::parser::ParserError> for CodegenError {
    fn from(error: sqlparser::parser::ParserError) -> CodegenError {
        CodegenError::ParserError(error)
    }
}

impl From<postgres::Error> for CodegenError {
    fn from(error: postgres::Error) -> CodegenError {
        CodegenError::PostgresError(error)
    }
}
