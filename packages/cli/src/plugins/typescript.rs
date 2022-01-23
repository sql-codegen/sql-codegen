use super::{Plugin, PluginResult};
use crate::data;
use convert_case::{Case, Casing};

#[derive(Debug)]
pub struct TypeScriptPlugin {
    name: &'static str,
}

impl TypeScriptPlugin {
    pub fn new() -> TypeScriptPlugin {
        TypeScriptPlugin { name: "typescript" }
    }

    pub fn get_table_type_name(&self, table: &data::Table) -> String {
        table.name.to_case(Case::Pascal)
    }

    pub fn get_column_field_type_name(&self, column: &data::Column) -> String {
        let sql_type = column.sql_type.to_string().replace("[]", "");
        let ts_type = match sql_type.as_str() {
            "BOOLEAN" => "Scalars[\"Boolean\"]".to_string(),
            "BIGINT" => "Scalars[\"BigInt\"]".to_string(),
            "HSTORE" => "Scalars[\"Hstore\"]".to_string(),
            "TEXT" => "Scalars[\"Text\"]".to_string(),
            "UUID" => "Scalars[\"Uuid\"]".to_string(),
            sql_type if sql_type.contains("CHAR") => "Scalars[\"Char\"]".to_string(),
            sql_type if sql_type.contains("DOUBLE") => "Scalars[\"Double\"]".to_string(),
            sql_type if sql_type.contains("ENUM") => "unknown".to_string(),
            sql_type if sql_type.contains("INT") => "Scalars[\"Int\"]".to_string(),
            sql_type if sql_type.contains("JSON") => "Scalars[\"Json\"]".to_string(),
            sql_type if sql_type.contains("REAL") => "Scalars[\"Real\"]".to_string(),
            sql_type if sql_type.contains("TIMESTAMP") => "Scalars[\"Timestamp\"]".to_string(),
            _ => sql_type.to_string(),
        };
        format!(
            "{ts_type}{or_null}",
            ts_type = ts_type,
            or_null = if column.is_not_null { "" } else { " | null" }
        )
    }

    pub fn get_column_field_name(&self, column: &data::Column) -> String {
        column.name.clone()
    }

    fn get_table_type_definition(&self, table: &data::Table) -> String {
        let name = self.get_table_type_name(table);
        let fields = table
            .columns
            .iter()
            .map(|column| self.get_column_field_definition(column))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "\
            export type {name} = {{\n\
            {fields}\n\
            }};"
        )
    }

    fn get_column_field_definition(&self, column: &data::Column) -> String {
        let name = self.get_column_field_name(column);
        let ts_type = self.get_column_field_type_name(column);
        format!("\t{name}: {ts_type};")
    }

    fn get_scalars(&self) -> Vec<(String, String)> {
        vec![
            ("BigInt".to_string(), "BigInt".to_string()),
            ("Boolean".to_string(), "boolean".to_string()),
            ("Char".to_string(), "string".to_string()),
            ("Double".to_string(), "number".to_string()),
            ("Hstore".to_string(), "Record<string, unknown>".to_string()),
            ("Int".to_string(), "number".to_string()),
            ("Json".to_string(), "Record<string, unknown>".to_string()),
            ("Real".to_string(), "number".to_string()),
            ("Text".to_string(), "string".to_string()),
            ("Timestamp".to_string(), "Date".to_string()),
            ("Uuid".to_string(), "string".to_string()),
        ]
    }

    fn get_codes(&self, tables: &Vec<data::Table>) -> Vec<String> {
        tables
            .iter()
            .map(|table| self.get_table_type_definition(table))
            .collect::<Vec<String>>()
    }
}

impl Plugin for TypeScriptPlugin {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> PluginResult {
        PluginResult::from(
            self.get_codes(&data.database.tables),
            vec![],
            self.get_scalars(),
        )
    }
}
