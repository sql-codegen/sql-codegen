use super::table::Table;
use crate::error;
use sqlparser::{ast::Statement, dialect::PostgreSqlDialect, parser::Parser};
use std::env;
use std::fs;

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
        relative_schema_file_path: &str,
    ) -> Result<Database, error::CodegenError> {
        let current_dir = env::current_dir()?;
        let absolute_schema_file_path = current_dir.join(relative_schema_file_path);
        let schema_ddl = fs::read_to_string(absolute_schema_file_path)?;
        let dialect = PostgreSqlDialect {};
        let ast = Parser::parse_sql(&dialect, &schema_ddl).unwrap();
        Ok(Self::from_ast(&ast))
    }

    pub fn from_ast(ast: &Vec<Statement>) -> Database {
        let tables: Vec<Table> = ast
            .iter()
            .filter_map(|statement| match statement {
                Statement::CreateTable { .. } => Some(Table::from_statement(statement)),
                _ => None,
            })
            .collect();
        Database::new("public".to_string(), tables)
    }

    pub fn to_string(&self) -> String {
        format!(
            "Database = {}\nTables = {}",
            self.name,
            self.tables
                .iter()
                .map(|table| table.to_string())
                .collect::<Vec<String>>()
                .join(",\n")
        )
    }

    pub fn has_table(&self, table_name: &str) -> bool {
        self.tables.iter().any(|table| table.name == table_name)
    }

    pub fn find_table(&self, table_name: &str) -> Option<&Table> {
        self.tables.iter().find(|table| table.name == table_name)
    }
}
