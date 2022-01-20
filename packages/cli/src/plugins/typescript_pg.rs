use super::{Plugin, PluginResult, TypeScriptGenericSdkPlugin, TypeScriptPlugin};
use crate::data;

#[derive(Debug)]
pub struct TypeScriptPgPlugin<'a> {
    name: &'static str,
    typescript_plugin: &'a TypeScriptPlugin,
    typescript_generic_sdk_plugin: &'a TypeScriptGenericSdkPlugin<'a>,
}

impl<'a> TypeScriptPgPlugin<'a> {
    pub fn new(
        typescript_plugin: &'a TypeScriptPlugin,
        typescript_generic_sdk_plugin: &'a TypeScriptGenericSdkPlugin,
    ) -> TypeScriptPgPlugin<'a> {
        TypeScriptPgPlugin {
            name: "typescript-pg",
            typescript_plugin,
            typescript_generic_sdk_plugin,
        }
    }

    fn get_codes(&self) -> Vec<String> {
        vec!["export const getPgSdk = (client: Client) => getSdk(async (query, variables) => (await client.query(query, variables as any)).rows as any);\n".to_string()]
    }

    fn get_imports(&self) -> Vec<String> {
        vec!["import type { Client } from \"pg\";".to_string()]
    }
}

impl<'a> Plugin for TypeScriptPgPlugin<'a> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, _data: &data::Data) -> PluginResult {
        PluginResult::from(self.get_codes(), self.get_imports(), vec![])
    }
}
