use super::Plugin;
use crate::data;
use convert_case::{Case, Casing};

const COMMON_TYPES: &str = "export type Uuid = string;\n";

#[derive(Debug)]
pub struct TypeScriptPlugin {
    name: &'static str,
}

impl TypeScriptPlugin {
    pub fn new() -> TypeScriptPlugin {
        TypeScriptPlugin { name: "typescript" }
    }

    pub fn get_indentation(&self) -> String {
        String::from("\t")
    }

    pub fn get_type_name_from_table(&self, table: &data::Table) -> String {
        table.name.to_case(Case::Pascal)
    }

    pub fn get_field_type_name_from_column(&self, column: &data::Column) -> String {
        let sql_type = column.sql_type.to_string().replace("[]", "");
        let ts_type = match sql_type.as_str() {
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
        format!(
            "{ts_type}{or_null}",
            ts_type = ts_type,
            or_null = if column.is_not_null { "" } else { " | null" }
        )
    }

    pub fn get_field_name_from_column(&self, column: &data::Column) -> String {
        column.name.clone()
    }

    fn get_type_definition_from_table(&self, table: &data::Table) -> String {
        let fields = table
            .columns
            .iter()
            .map(|column| self.get_field_definition_from_column(column))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "export type {name} = {{\n{fields}\n}};",
            name = self.get_type_name_from_table(table),
            fields = fields
        )
    }

    fn get_field_definition_from_column(&self, column: &data::Column) -> String {
        format!(
            "{indentation}{name}: {type};",
            indentation = self.get_indentation(),
            name = self.get_field_name_from_column(column),
            type = self.get_field_type_name_from_column(column),
        )
    }

    fn generate(&self, tables: &Vec<data::Table>) -> String {
        format!(
            "{common_types}\n{types}\n",
            common_types = COMMON_TYPES,
            types = tables
                .iter()
                .map(|table| self.get_type_definition_from_table(table))
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}

impl Plugin for TypeScriptPlugin {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> String {
        self.generate(&data.database.tables)
    }
}
