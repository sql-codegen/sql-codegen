use convert_case::{Case, Casing};

use super::{Plugin, TypeScriptOperationsPlugin, TypeScriptPlugin};
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

    fn get_function_name_from_query(&self, query: &data::Query) -> String {
        query
            .path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_case(Case::Camel)
    }

    fn get_function_definition_from_query(&self, query: &data::Query) -> String {
        format!(
            "{indentation}{indentation}{function_name}: (variables?: {variables_type_name}): Promise<{result_type_name}[]> => requester<{result_type_name}[], {variables_type_name}>({document_variable_name}, variables),",
            function_name = self.get_function_name_from_query(query),
            indentation = self.typescript_plugin.get_indentation(),
            result_type_name = self.typescript_operation_plugin.get_result_type_name(query),
            variables_type_name = self.typescript_operation_plugin.get_variables_type_name(query),
            document_variable_name = self.typescript_operation_plugin.get_ddl_variable_name(query),
        )
    }
}

impl<'a> Plugin for TypeScriptGenericSdkPlugin<'a> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> String {
        let functions = data
            .queries
            .iter()
            .map(|query| self.get_function_definition_from_query(query))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "// TypeScript Generic SDK Plugin\n\nexport type Requester = <R, V>(query: string, variables?: V) => Promise<R>;\nexport const getSdk = (requester: Requester) => {{\n{indentation}return {{\n{functions}\n{indentation}}};\n}};\n",
            indentation = self.typescript_plugin.get_indentation(),
            functions = functions
        )
    }
}
