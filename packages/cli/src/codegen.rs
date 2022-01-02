use std::{env, fs};

use crate::config::Config;

#[derive()]
pub struct Codegen<'a> {
    config: &'a Config,
}

impl<'a> Codegen<'a> {
    pub fn new(config: &'a Config) -> Codegen {
        Codegen { config }
    }

    pub fn load_queries(&self) -> Vec<String> {
        let current_dir = env::current_dir().unwrap();
        let queries_file_path = current_dir.join(&self.config.queries);
        if !queries_file_path.exists() {
            panic!("Queries file not found");
        }
        let content = fs::read_to_string(queries_file_path).expect("Error reading query files");
        let queries = content
            .split(";")
            .filter(|query| query.len() > 0)
            .map(|query| query.to_string())
            .collect::<Vec<String>>();
        queries
    }

    pub fn load_schema_ddl(&self) -> String {
        let current_dir = env::current_dir().unwrap();
        let schema_file_path = current_dir.join(&self.config.schema);
        if !schema_file_path.exists() {
            panic!("Schema file not found");
        }
        let content = fs::read_to_string(schema_file_path).expect("Error reading query files");
        content
    }
}
