use super::{Plugin, PluginResult, TypeScriptOperationsPlugin};
use crate::data;
use convert_case::{Case, Casing};

#[derive(Debug)]
pub struct TypeScriptGenericSdkPlugin<'a> {
    name: &'static str,
    typescript_operation_plugin: &'a TypeScriptOperationsPlugin<'a>,
}

impl<'a> TypeScriptGenericSdkPlugin<'a> {
    pub fn new(
        typescript_operation_plugin: &'a TypeScriptOperationsPlugin,
    ) -> TypeScriptGenericSdkPlugin<'a> {
        TypeScriptGenericSdkPlugin {
            name: "typescript-generic-sdk",
            typescript_operation_plugin,
        }
    }

    fn get_query_mode_definition(&self) -> String {
        "export type RowMode = \"array\" | \"object\";".to_string()
    }

    fn get_request_params_definition(&self) -> String {
        "\
        export type RequesterParams<V> = {\n\
        \tquery: string;\n\
        \tvariables?: V;\n\
        \trowMode?: RowMode;\n\
        };"
        .to_string()
    }

    fn get_requester_definition(&self) -> String {
        "export type Requester = <R, V>(params: RequesterParams<V>) => Promise<R>;".to_string()
    }

    fn get_fetch_array_result_params_definition(&self) -> String {
        "\
        export type FetchArrayResultParams<V> = {\n\
        \tquery?: string;\n\
        \tvariables?: V;\n\
        };"
        .to_string()
    }

    fn get_fetch_object_result_params_definition(&self) -> String {
        "\
        export type FetchObjectResultParams<V> = {\n\
        \tquery?: string;\n\
        \tvariables?: V;\n\
        };"
        .to_string()
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
        let function_name = self.get_query_function_name(query);
        let array_result_type_name = self
            .typescript_operation_plugin
            .get_query_array_result_type_name(query);
        let object_result_type_name = self
            .typescript_operation_plugin
            .get_query_object_result_type_name(query);
        let variables_type_name = self
            .typescript_operation_plugin
            .get_variables_type_name(query);
        let document_variable_name = self
            .typescript_operation_plugin
            .get_ddl_variable_name(query);
        format!(
            "\t\t{function_name}: fetchObjectResult<{object_result_type_name}, {variables_type_name}>({document_variable_name}),\n\
            \t\t{function_name}AsArray: fetchArrayResult<{array_result_type_name}, {variables_type_name}>({document_variable_name}),"
        )
    }

    fn get_fetch_array_result_definition(&self) -> String {
        "const fetchArrayResult = <R, V>(query: string) => (params?: FetchArrayResultParams<V>): Promise<R[]> => requester<R[], V>({ ...params, query, rowMode: \"array\" });".to_string()
    }

    fn get_fetch_object_result_definition(&self) -> String {
        "const fetchObjectResult = <R, V>(query: string) => (params?: FetchObjectResultParams<V>): Promise<R[]> => requester<R[], V>({ ...params, query, rowMode: \"object\" });".to_string()
    }

    fn get_get_sdk_definition(&self, queries: &Vec<data::Query>) -> String {
        let fetch_array_result = self.get_fetch_array_result_definition();
        let fetch_object_result = self.get_fetch_object_result_definition();
        let functions = queries
            .iter()
            .map(|query| self.get_query_function_definition(query))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "\
            export const getSdk = (requester: Requester) => {{\n\
            \t{fetch_array_result}\n\
            \t{fetch_object_result}\n\
            \n\
            \treturn {{\n\
            {functions}\n\
            \t}};\n\
            }};",
        )
    }

    fn get_codes(&self, data: &data::Data) -> Vec<String> {
        vec![
            self.get_query_mode_definition(),
            self.get_request_params_definition(),
            self.get_requester_definition(),
            self.get_fetch_array_result_params_definition(),
            self.get_fetch_object_result_params_definition(),
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
