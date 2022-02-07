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

    pub fn get_array_result_element_name(&self, selection: &data::Selection) -> String {
        selection.column_name.clone()
    }

    pub fn get_array_result_element_definition(&self, selection: &data::Selection) -> String {
        let name = self.get_array_result_element_name(selection);
        let ts_type = self
            .typescript_plugin
            .get_column_field_type_name(selection.column);
        format!("\t{name}: {ts_type},")
    }

    pub fn get_object_result_field_name(&self, selection: &data::Selection) -> String {
        selection.column_name.clone()
    }

    pub fn get_object_result_field_definition(&self, selection: &data::Selection) -> String {
        let name = self.get_object_result_field_name(selection);
        let ts_type = self
            .typescript_plugin
            .get_column_field_type_name(selection.column);
        format!("\t{name}: {ts_type};")
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

    pub fn get_query_array_result_type_name(&self, query: &data::Query) -> String {
        let file_stem = self.get_file_stem(query);
        let prefix = file_stem.to_case(Case::Pascal);
        format!("{prefix}QueryArrayResult")
    }

    pub fn get_query_object_result_type_name(&self, query: &data::Query) -> String {
        let file_stem = self.get_file_stem(query);
        let prefix = file_stem.to_case(Case::Pascal);
        format!("{prefix}QueryObjectResult")
    }

    pub fn get_query_array_result_type_definition(&self, query: &data::Query) -> String {
        let array_result_type_name = self.get_query_array_result_type_name(query);
        let fields = query
            .projection
            .selections
            .iter()
            .map(|selection| self.get_array_result_element_definition(selection))
            .collect::<Vec<String>>()
            .join("\n");
        format!("export type {array_result_type_name} = readonly [\n{fields}\n];")
    }

    pub fn get_query_object_result_type_definition(&self, query: &data::Query) -> String {
        let object_result_type_name = self.get_query_object_result_type_name(query);
        let fields = query
            .projection
            .selections
            .iter()
            .enumerate()
            // Filter out duplicates and only leave the last one.
            .filter(|(index, selection_a)| {
                let has_duplicate = query
                    .projection
                    .selections
                    .iter()
                    .skip(index + 1)
                    .any(|selection_b| selection_a.column_name == selection_b.column_name);
                !has_duplicate
            })
            .map(|(_index, selection)| self.get_object_result_field_definition(selection))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "\
            export type {object_result_type_name} = Readonly<{{\n\
            {fields}\n\
            }}>;"
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
            "export type {variables_type_name} = [];",
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
            self.get_query_array_result_type_definition(query),
            self.get_query_object_result_type_definition(query),
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
