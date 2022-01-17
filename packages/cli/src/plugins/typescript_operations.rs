use super::{Plugin, TypeScriptPlugin};
use crate::data;
use crate::projection;
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

    pub fn get_field_definition_from_projection(
        &self,
        projection: &projection::Projection,
    ) -> String {
        format!(
            "{indentation}{name}: {type},",
            indentation = self.typescript_plugin.get_indentation(),
            name = projection.column_name,
            type = self.typescript_plugin.get_field_type_name_from_column(projection.column)
        )
    }

    pub fn get_result_type_name(&self, query: &data::Query) -> String {
        let file_stem = query
            .path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let result_type_name = format!("{}Result", file_stem.to_case(Case::Pascal));
        result_type_name
    }

    pub fn get_type_definitions_from_query(&self, query: &data::Query) -> String {
        let result_type_name = self.get_result_type_name(query);
        let fields = query
            .projections
            .iter()
            .map(|projection| self.get_field_definition_from_projection(projection))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "// Types for the \"{path}\" file\nexport type {result_type_name} = {{\n{fields}\n}};",
            path = query.path.to_str().unwrap(),
            result_type_name = result_type_name,
            fields = fields
        )
    }
}

impl<'a> Plugin for TypeScriptOperationsPlugin<'a> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> String {
        let names = data
            .queries
            .iter()
            .map(|query| self.get_type_definitions_from_query(query))
            .collect::<Vec<String>>()
            .join("\n\n");
        format!("// TypeScript Operations Plugin\n\n{}", names)
    }
}
