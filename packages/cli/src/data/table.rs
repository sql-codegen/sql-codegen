use super::column::Column;
use crate::utils;
use sqlparser::ast::Statement;

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

    pub fn from_statement(statement: &Statement) -> Table {
        if let Statement::CreateTable { columns, name, .. } = statement {
            let columns: Vec<Column> = columns
                .iter()
                .map(|column| Column::from_column_definition(column))
                .collect();
            return Table::new(utils::object_name_to_string(name), columns);
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
