use super::PluginResult;
use super::{Plugin, TypeScriptPlugin};
use crate::data;
use convert_case::{Case, Casing};

#[derive(Debug)]
pub struct TypeScriptOperationsPlugin<'a> {
    name: &'static str,
    typescript_plugin: &'a TypeScriptPlugin,
}

impl<'a> TypeScriptOperationsPlugin<'a> {
    pub fn new(typescript_plugin: &TypeScriptPlugin) -> TypeScriptOperationsPlugin {
        TypeScriptOperationsPlugin {
            name: "typescript-operations",
            typescript_plugin,
        }
    }

    pub fn get_field_definition(&self, selection: &data::Selection) -> String {
        format!(
            "\t{name}: {type},",
            name = selection.column_name,
            type = self.typescript_plugin.get_column_field_type_name(selection.column)
        )
    }

    fn get_file_stem(&self, query: &data::Query) -> String {
        query
            .path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_query_result_type_name(&self, query: &data::Query) -> String {
        let file_stem = self.get_file_stem(query);
        let result_type_name = format!("{}QueryResult", file_stem.to_case(Case::Pascal));
        result_type_name
    }

    pub fn get_query_result_type_definition(&self, query: &data::Query) -> String {
        let result_type_name = self.get_query_result_type_name(query);
        let fields = query
            .projection
            .selections
            .iter()
            .map(|selection| self.get_field_definition(selection))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "export type {result_type_name} = {{\n{fields}\n}};",
            fields = fields,
            result_type_name = result_type_name
        )
    }

    pub fn get_variables_type_name(&self, query: &data::Query) -> String {
        let file_stem = self.get_file_stem(query);
        let variables_type_name = format!("{}QueryVariables", file_stem.to_case(Case::Pascal));
        variables_type_name
    }

    pub fn get_variables_type_definition(&self, query: &data::Query) -> String {
        let variables_type_name = self.get_variables_type_name(query);
        format!(
            "export type {variables_type_name} = {{}};",
            variables_type_name = variables_type_name
        )
    }

    pub fn get_ddl_variable_name(&self, query: &data::Query) -> String {
        let file_stem = self.get_file_stem(query);
        format!("{}QueryDdl", file_stem.to_case(Case::Pascal))
    }

    pub fn get_ddl_variable_value(&self, query: &data::Query) -> String {
        query.ddl.clone()
    }

    pub fn get_ddl_variable(&self, query: &data::Query) -> String {
        let ddl_variable_name = self.get_ddl_variable_name(query);
        let ddl_variable_value = self.get_ddl_variable_value(query);
        format!(
            "const {ddl_variable_name} = `{ddl_variable_value}`;",
            ddl_variable_name = ddl_variable_name,
            ddl_variable_value = ddl_variable_value
        )
    }

    pub fn get_type_definitions(&self, query: &data::Query) -> Vec<String> {
        vec![
            self.get_query_result_type_definition(query),
            self.get_variables_type_definition(query),
            self.get_ddl_variable(query),
        ]
    }

    pub fn get_codes(&self, data: &data::Data) -> Vec<String> {
        data.queries
            .iter()
            .map(|query| self.get_type_definitions(query))
            .flatten()
            .collect::<Vec<String>>()
    }
}

impl<'a> Plugin for TypeScriptOperationsPlugin<'a> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> PluginResult {
        PluginResult::from(self.get_codes(data), vec![], vec![])
    }
}
