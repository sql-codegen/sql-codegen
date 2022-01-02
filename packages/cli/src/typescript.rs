use crate::schema;
use convert_case::{Case, Casing};
use std::fs;

const COMMON_CODE: &str = "";

const COMMON_TYPES: &str = "export type Uuid = string;";

fn sql_type_to_ts_type(sql_type: &str, is_array: bool) -> String {
    let ts_type = match sql_type {
        sql_type if sql_type.contains("[]") => {
            sql_type_to_ts_type(&sql_type.replace("[]", ""), true)
        }
        "bigint" => "BigInt".to_string(),
        "hstore" => "Record<string, unknown>".to_string(),
        "text" => "string".to_string(),
        "uuid" => "Uuid".to_string(),
        sql_type if sql_type.contains("char") => "string".to_string(),
        sql_type if sql_type.contains("double") => "number".to_string(),
        sql_type if sql_type.contains("enum") => "unknown".to_string(),
        sql_type if sql_type.contains("int") => "number".to_string(),
        sql_type if sql_type.contains("json") => "Record<string, unknown>".to_string(),
        sql_type if sql_type.contains("real") => "number".to_string(),
        sql_type if sql_type.contains("timestamp") => "Date".to_string(),
        _ => sql_type.to_string(),
    };
    if is_array {
        return format!("{}[]", ts_type);
    }
    ts_type
}

fn get_ts_code_for_table(table: &schema::Table) -> String {
    let fields = table
        .columns
        .iter()
        .map(get_ts_code_for_column)
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        "export type {name} = {{\n{fields}\n}};",
        name = table.name.to_case(Case::Pascal),
        fields = fields
    )
}

fn get_ts_code_for_column(column: &schema::Column) -> String {
    format!(
        "  {name}: {type}{or_null};",
        name = column.name.to_case(Case::Camel),
        type = sql_type_to_ts_type(&column.sql_type.to_string(), false),
        or_null = if column.is_not_null { "" } else { " | null" }
    )
}

pub fn generate(tables: &Vec<schema::Table>) -> () {
    let types = format!(
        "{common_code}\n\n{common_types}\n\n{types}\n",
        common_code = COMMON_CODE,
        common_types = COMMON_TYPES,
        types = tables
            .iter()
            .map(get_ts_code_for_table)
            .collect::<Vec<String>>()
            .join("\n\n"),
    );
    fs::create_dir_all("generated").expect("Error creating directory");
    fs::write("generated/types.ts", types).expect("Error creating types.ts file");
}
