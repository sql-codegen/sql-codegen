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

    fn run(&self, _data: &data::Data) -> String {
        String::from("// TypeScript Operations Plugin\n")
    }
}
