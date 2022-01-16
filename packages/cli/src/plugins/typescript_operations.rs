use super::Plugin;
use crate::data;
use convert_case::{Case, Casing};

pub struct TypeScriptOperationsPlugin {
    name: &'static str,
}

impl TypeScriptOperationsPlugin {
    pub fn new() -> Self {
        Self {
            name: "typescript-operations",
        }
    }

    fn get_result_type_name(&self, query: &data::Query) -> String {
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
}

impl Plugin for TypeScriptOperationsPlugin {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, data: &data::Data) -> String {
        let names = data
            .queries
            .iter()
            .map(|query| {
                let result_type_name = self.get_result_type_name(query);
                let fields = query.projections.iter().map(|projection| {
                    format!(
                        "\t{name}: {type},",
                        name = projection.column_name, type = projection.column.get_ts_type()
                    )
                }).collect::<Vec<String>>().join("\n");
                format!(
                    "// Types for the \"{path}\" file\nexport type {result_type_name} = {{\n{fields}\n}};",
                    path = query.path.to_str().unwrap(),
                    result_type_name = result_type_name,
                    fields = fields
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        format!("// TypeScript Operations Plugin\n\n{}", names)
    }
}
