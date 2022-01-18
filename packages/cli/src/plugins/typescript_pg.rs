use convert_case::{Case, Casing};

use super::{Plugin, TypeScriptGenericSdkPlugin, TypeScriptPlugin};
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
}

impl<'a> Plugin for TypeScriptPgPlugin<'a> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self, _data: &data::Data) -> String {
        format!("import type {{ Client }} from \"pg\";\n\nexport const getPgSdk = (client: Client) => getSdk(async (query, variables) => (await client.query(query, variables as any)).rows as any);\n")
    }
}
