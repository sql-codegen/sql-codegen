use super::Plugin;
use crate::schema;
use convert_case::{Case, Casing};

const COMMON_CODE: &str = "";

const COMMON_TYPES: &str = "export type Uuid = string;";

pub struct TypeScriptPlugin {
    name: &'static str,
}

impl TypeScriptPlugin {
    pub fn new() -> Self {
        Self { name: "typescript" }
    }
    pub fn get_type_name_for_table(&self, table: &schema::Table) -> String {
        table.name.to_case(Case::Pascal)
    }

    pub fn get_type_name_for_column(&self, column: &schema::Column) -> String {
        column.name.to_case(Case::Camel)
    }

    fn sql_type_to_ts_type(&self, sql_type: &str) -> String {
        let ts_type = match sql_type {
            sql_type if sql_type.contains("[]") => {
                let ts_type = self.sql_type_to_ts_type(&sql_type.replace("[]", ""));
                return format!("{}[]", ts_type);
            }
            "BOOLEAN" => "boolean".to_string(),
            "BIGINT" => "BigInt".to_string(),
            "HSTORE" => "Record<string, unknown>".to_string(),
            "TEXT" => "string".to_string(),
            "UUID" => "Uuid".to_string(),
            sql_type if sql_type.contains("CHAR") => "string".to_string(),
            sql_type if sql_type.contains("DOUBLE") => "number".to_string(),
            sql_type if sql_type.contains("ENUM") => "unknown".to_string(),
            sql_type if sql_type.contains("INT") => "number".to_string(),
            sql_type if sql_type.contains("JSON") => "Record<string, unknown>".to_string(),
            sql_type if sql_type.contains("REAL") => "number".to_string(),
            sql_type if sql_type.contains("TIMESTAMP") => "Date".to_string(),
            _ => sql_type.to_string(),
        };
        ts_type
    }

    fn get_ts_code_for_table(&self, table: &schema::Table) -> String {
        let fields = table
            .columns
            .iter()
            .map(|column| self.get_ts_code_for_column(column))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "export type {name} = {{\n{fields}\n}};",
            name = self.get_type_name_for_table(table),
            fields = fields
        )
    }

    fn get_ts_code_for_column(&self, column: &schema::Column) -> String {
        format!(
            "  {name}: {type}{or_null};",
            name = self.get_type_name_for_column(column),
            type = self.sql_type_to_ts_type(&column.sql_type.to_string()),
            or_null = if column.is_not_null { "" } else { " | null" }
        )
    }

    fn generate(&self, tables: &Vec<schema::Table>) -> String {
        format!(
            "{common_code}\n\n{common_types}\n\n{types}\n",
            common_code = COMMON_CODE,
            common_types = COMMON_TYPES,
            types = tables
                .iter()
                .map(|table| self.get_ts_code_for_table(table))
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}

impl Plugin for TypeScriptPlugin {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, database: &schema::Database) -> String {
        self.generate(&database.tables)
    }
}
