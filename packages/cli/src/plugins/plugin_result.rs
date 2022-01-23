#[derive(Debug)]
pub struct PluginResult {
    pub codes: Vec<String>,
    pub imports: Vec<String>,
    pub scalars: Vec<(String, String)>,
}

impl PluginResult {
    pub fn new() -> PluginResult {
        PluginResult {
            codes: vec![],
            imports: vec![],
            scalars: vec![],
        }
    }

    pub fn from(
        codes: Vec<String>,
        imports: Vec<String>,
        scalars: Vec<(String, String)>,
    ) -> PluginResult {
        PluginResult {
            codes,
            imports,
            scalars,
        }
    }

    pub fn append(&mut self, plugin_result: &mut PluginResult) {
        self.codes.append(&mut plugin_result.codes);
        self.imports.append(&mut plugin_result.imports);
        self.scalars.append(&mut plugin_result.scalars);
    }

    fn format_imports(&self) -> String {
        self.imports.join("\n")
    }

    fn format_scalars(&self) -> String {
        let scalars = self
            .scalars
            .iter()
            .map(|(sql_type, dest_type)| format!("\t{sql_type}: {dest_type};"))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "\
            export type Scalars = {{\n\
            {scalars}\n\
            }};"
        )
    }

    fn format_codes(&self) -> String {
        self.codes.join("\n\n")
    }

    pub fn to_string(&self) -> String {
        let imports = self.format_imports();
        let scalars = self.format_scalars();
        let codes = self.format_codes();
        format!(
            "\
            {imports}\n\
            \n\
            {scalars}\n\
            \n\
            {codes}"
        )
    }
}
