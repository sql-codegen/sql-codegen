use crate::schema;
use convert_case::{Case, Casing};

const COMMON_CODE: &str = "";

const COMMON_TYPES: &str = "export type Uuid = string;";

pub struct TypeScriptPlugin {}

impl TypeScriptPlugin {
    pub fn get_type_name_for_table(table: &schema::Table) -> String {
        table.name.to_case(Case::Pascal)
    }

    pub fn get_type_name_for_column(column: &schema::Column) -> String {
        column.name.to_case(Case::Camel)
    }

    fn sql_type_to_ts_type(sql_type: &str) -> String {
        let ts_type = match sql_type {
            sql_type if sql_type.contains("[]") => {
                let ts_type = Self::sql_type_to_ts_type(&sql_type.replace("[]", ""));
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

    fn get_ts_code_for_table(table: &schema::Table) -> String {
        let fields = table
            .columns
            .iter()
            .map(|column| Self::get_ts_code_for_column(column))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "export type {name} = {{\n{fields}\n}};",
            name = Self::get_type_name_for_table(table),
            fields = fields
        )
    }

    fn get_ts_code_for_column(column: &schema::Column) -> String {
        format!(
            "  {name}: {type}{or_null};",
            name = Self::get_type_name_for_column(column),
            type = Self::sql_type_to_ts_type(&column.sql_type.to_string()),
            or_null = if column.is_not_null { "" } else { " | null" }
        )
    }

    fn generate(tables: &Vec<schema::Table>) -> String {
        format!(
            "{common_code}\n\n{common_types}\n\n{types}\n",
            common_code = COMMON_CODE,
            common_types = COMMON_TYPES,
            types = tables
                .iter()
                .map(|table| Self::get_ts_code_for_table(table))
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }

    pub fn run(database: &schema::Database) -> String {
        Self::generate(&database.tables)
    }
}
