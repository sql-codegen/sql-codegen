use crate::error::CodegenError;
use sqlparser::{ast::Statement, dialect::PostgreSqlDialect, parser::Parser};
use std::{fs, path::PathBuf};

pub struct Data {
    pub tables_ast: Vec<Statement>,
    pub queries_ast: Vec<Statement>,
}

impl Data {
    pub fn new(
        schema_file_path: PathBuf,
        queries_file_path: PathBuf,
    ) -> Result<Self, CodegenError> {
        let dialect = PostgreSqlDialect {};

        let tables_ddl = fs::read_to_string(schema_file_path)?;
        let tables_ast = Parser::parse_sql(&dialect, &tables_ddl)?;

        let queries_ddl = fs::read_to_string(queries_file_path)?;
        let queries_ast = Parser::parse_sql(&dialect, &queries_ddl)?;

        Ok(Self {
            tables_ast,
            queries_ast,
        })
    }
}
