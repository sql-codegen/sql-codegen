use super::Plugin;
use crate::data;

pub struct TypeScriptOperationsPlugin {
    name: &'static str,
}

impl TypeScriptOperationsPlugin {
    pub fn new() -> Self {
        Self {
            name: "typescript-operations",
        }
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
            .map(|query| format!("// {}", query.name))
            .collect::<Vec<String>>()
            .join("\n\n");
        format!("// TypeScript Operations Plugin\n\n{}", names).to_string()
    }
}
