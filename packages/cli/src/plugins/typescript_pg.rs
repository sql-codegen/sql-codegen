use super::{Plugin, PluginResult};
use crate::data;

#[derive(Debug)]
pub struct TypeScriptPgPlugin {
    name: &'static str,
}

impl TypeScriptPgPlugin {
    pub fn new() -> TypeScriptPgPlugin {
        TypeScriptPgPlugin {
            name: "typescript-pg",
        }
    }

    fn get_codes(&self) -> Vec<String> {
        vec!["export const getPgSdk = (client: Client) => getSdk(async ({ query, variables, rowMode }) => (await client.query({ text: query, values: variables as any, rowMode: rowMode as any }) as any).rows);\n".to_string()]
    }

    fn get_imports(&self) -> Vec<String> {
        vec!["import type { Client } from \"pg\";".to_string()]
    }
}

impl Plugin for TypeScriptPgPlugin {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, _data: &data::Data) -> PluginResult {
        PluginResult::from(self.get_codes(), self.get_imports(), vec![])
    }
}
