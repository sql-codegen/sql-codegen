use convert_case::{Case, Casing};

use super::{Plugin, PluginResult, TypeScriptOperationsPlugin, TypeScriptPlugin};
use crate::data;

#[derive(Debug)]
pub struct TypeScriptGenericSdkPlugin<'a> {
    name: &'static str,
    typescript_plugin: &'a TypeScriptPlugin,
    typescript_operation_plugin: &'a TypeScriptOperationsPlugin<'a>,
}

impl<'a> TypeScriptGenericSdkPlugin<'a> {
    pub fn new(
        typescript_plugin: &'a TypeScriptPlugin,
        typescript_operation_plugin: &'a TypeScriptOperationsPlugin,
    ) -> TypeScriptGenericSdkPlugin<'a> {
        TypeScriptGenericSdkPlugin {
            name: "typescript-generic-sdk",
            typescript_plugin,
            typescript_operation_plugin,
        }
    }
    fn get_requester_definition(&self) -> String {
        format!("export type Requester = <R, V>(query: string, variables?: V) => Promise<R>;")
    }

    fn get_query_function_name(&self, query: &data::Query) -> String {
        query
            .path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_case(Case::Camel)
    }

    fn get_query_function_definition(&self, query: &data::Query) -> String {
        format!(
            "\t\t{function_name}: (variables?: {variables_type_name}): Promise<{result_type_name}[]> => requester<{result_type_name}[], {variables_type_name}>({document_variable_name}, variables),",
            function_name = self.get_query_function_name(query),
            result_type_name = self.typescript_operation_plugin.get_query_result_type_name(query),
            variables_type_name = self.typescript_operation_plugin.get_variables_type_name(query),
            document_variable_name = self.typescript_operation_plugin.get_ddl_variable_name(query),
        )
    }

    fn get_get_sdk_definition(&self, queries: &Vec<data::Query>) -> String {
        let functions = queries
            .iter()
            .map(|query| self.get_query_function_definition(query))
            .collect::<Vec<String>>()
            .join("\n");
        format!("export const getSdk = (requester: Requester) => {{\n\treturn {{\n{functions}\n\t}};\n}};", functions = functions)
    }

    fn get_codes(&self, data: &data::Data) -> Vec<String> {
        vec![
            self.get_requester_definition(),
            self.get_get_sdk_definition(data.queries),
        ]
    }
}

impl<'a> Plugin for TypeScriptGenericSdkPlugin<'a> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> PluginResult {
        PluginResult::from(self.get_codes(data), vec![], vec![])
    }
}
