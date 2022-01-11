use super::{Database, Query};
use crate::error::CodegenError;
use sqlparser::{dialect::PostgreSqlDialect, parser::Parser};
use std::{fs, path::PathBuf};

pub struct Data {
    pub database: Database,
    pub queries: Vec<Query>,
}

impl Data {
    pub fn new(
        schema_file_path: &PathBuf,
        query_file_paths: &Vec<PathBuf>,
    ) -> Result<Data, CodegenError> {
        let dialect = PostgreSqlDialect {};

        let schema_ddl = fs::read_to_string(schema_file_path)?;
        let schema_ast = Parser::parse_sql(&dialect, &schema_ddl)?;
        let database = Database::from_ast(&schema_ast);

        let mut queries: Vec<Query> = vec![];
        for query_file_path in query_file_paths {
            let file_name = query_file_path.file_name().unwrap().to_str().unwrap();
            let query_ddl = fs::read_to_string(query_file_path)?;
            let query_ast = Parser::parse_sql(&dialect, &query_ddl)?;
            queries.push(Query::from_ast(file_name, &query_ast));
        }

        Ok(Data { database, queries })
    }
}
