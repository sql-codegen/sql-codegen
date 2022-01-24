use super::table::{self, Table};
use crate::error;
use sqlparser::{ast::Statement, dialect::PostgreSqlDialect, parser::Parser};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}

impl PartialEq for Database {
    fn eq(&self, other: &Database) -> bool {
        self.name == other.name && self.tables == other.tables
    }
}

impl Database {
    pub fn new(name: String, tables: Vec<Table>) -> Database {
        Database { name, tables }
    }

    pub fn from_schema_file_path(
        schema_file_path: PathBuf,
    ) -> Result<Database, error::CodegenError> {
        let dialect = PostgreSqlDialect {};
        let schema_ddl = fs::read_to_string(schema_file_path)?;
        let schema_ast = Parser::parse_sql(&dialect, &schema_ddl)?;
        Database::from_ast(&schema_ast)
    }

    fn from_ast(ast: &Vec<Statement>) -> Result<Database, error::CodegenError> {
        let tables = ast
            .iter()
            .filter_map(|statement| match statement {
                Statement::CreateTable { .. } => Some(Table::from_statement(statement)),
                _ => None,
            })
            .collect::<Result<Vec<table::Table>, error::CodegenError>>()?;
        Ok(Database::new("public".to_string(), tables))
    }

    #[allow(dead_code)]
    pub fn has_table(&self, table_name: &str) -> bool {
        self.tables.iter().any(|table| table.name == table_name)
    }

    pub fn find_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.name == table_name)
    }
}
