use crate::{codegen::Codegen, utils::object_name_to_string};
use sqlparser::{
    ast::{ColumnDef, DataType, Statement},
    dialect::PostgreSqlDialect,
    parser::Parser,
};

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

    pub fn from_codegen(codegen: &Codegen) -> Database {
        let schema_ddl = codegen.load_schema_ddl();
        let dialect = PostgreSqlDialect {};
        let ast = Parser::parse_sql(&dialect, &schema_ddl).unwrap();
        Database::from_statements(&ast)
    }

    pub fn from_statements(statements: &Vec<Statement>) -> Database {
        let tables: Vec<Table> = statements
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

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Clone for Table {
    fn clone(&self) -> Table {
        Table {
            name: self.name.clone(),
            columns: self.columns.clone(),
        }
    }
}

impl Table {
    pub fn new(name: String, columns: Vec<Column>) -> Table {
        Table { name, columns }
    }

    fn from_statement(statement: &Statement) -> Table {
        if let Statement::CreateTable { columns, name, .. } = statement {
            let columns: Vec<Column> = columns
                .iter()
                .map(|column| Column::from_column_definition(column))
                .collect();
            return Table::new(object_name_to_string(name), columns);
        }
        panic!("Expected a create table statement");
    }

    pub fn to_string(&self) -> String {
        format!(
            "Table = {}\nColumns = {}",
            self.name,
            self.columns
                .iter()
                .map(|column| column.to_string())
                .collect::<Vec<String>>()
                .join(",\n")
        )
    }

    pub fn has_column(&self, column_name: &str) -> bool {
        self.columns.iter().any(|column| column.name == column_name)
    }

    pub fn find_column(&self, column_name: &str) -> Option<&Column> {
        self.columns
            .iter()
            .find(|column| column.name == column_name)
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Table) -> bool {
        self.name == other.name && self.columns == other.columns
    }
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub sql_type: DataType,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub is_not_null: bool,
    pub default_value: Option<String>,
}

impl Clone for Column {
    fn clone(&self) -> Column {
        Column {
            name: self.name.clone(),
            sql_type: self.sql_type.clone(),
            is_primary_key: self.is_primary_key,
            is_unique: self.is_unique,
            is_not_null: self.is_not_null,
            default_value: self.default_value.clone(),
        }
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Column) -> bool {
        self.name == other.name
            && self.sql_type == other.sql_type
            && self.is_primary_key == other.is_primary_key
            && self.is_unique == other.is_unique
            && self.is_not_null == other.is_not_null
            && self.default_value == other.default_value
    }
}

impl Column {
    pub fn new(
        name: String,
        sql_type: DataType,
        is_primary_key: bool,
        is_unique: bool,
        is_not_null: bool,
        default_value: Option<String>,
    ) -> Column {
        Column {
            name,
            sql_type,
            is_primary_key,
            is_unique,
            is_not_null,
            default_value,
        }
    }

    pub fn from_column_definition(column_definition: &ColumnDef) -> Column {
        Column {
            name: column_definition.name.value.clone(),
            sql_type: column_definition.data_type.clone(),
            is_primary_key: false,
            is_unique: false,
            is_not_null: false,
            default_value: None,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Column = {}", self.name)
    }
}
