use super::Plugin;
use crate::data;
use convert_case::{Case, Casing};

const COMMON_CODE: &str = "// TypeScript Plugin\n";

const COMMON_TYPES: &str = "export type Uuid = string;\n";

pub struct TypeScriptPlugin {
    name: &'static str,
}

impl TypeScriptPlugin {
    pub fn new() -> Self {
        Self { name: "typescript" }
    }
    pub fn get_type_name_for_table(&self, table: &data::Table) -> String {
        table.name.to_case(Case::Pascal)
    }

    pub fn get_type_name_for_column(&self, column: &data::Column) -> String {
        column.name.to_case(Case::Camel)
    }

    fn get_ts_code_for_table(&self, table: &data::Table) -> String {
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

    fn get_ts_code_for_column(&self, column: &data::Column) -> String {
        format!(
            "  {name}: {type};",
            name = self.get_type_name_for_column(column),
            type = column.get_ts_type(),
        )
    }

    fn generate(&self, tables: &Vec<data::Table>) -> String {
        format!(
            "{common_code}\n{common_types}\n{types}\n",
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

    fn run(&self, data: &data::Data) -> String {
        self.generate(&data.database.tables)
    }
}
