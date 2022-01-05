use crate::config::Config;
use postgres::{Client, NoTls};
use std::{env, fmt::Debug, fs, path::PathBuf};

#[derive(Debug)]
pub struct Error<'a> {
    pub message: &'a str,
}

impl<'a> Error<'a> {
    fn new(message: &str) -> Error {
        Error { message }
    }
}

#[derive()]
pub struct Codegen<'a> {
    config: &'a Config,
    pub client: Client,
}

impl<'a> Codegen<'a> {
    pub fn new(config: &'a Config) -> Result<Codegen, Error> {
        let params = format!(
            "host={host} user={user} port={port} dbname={database} password={password}",
            host = config.connection.host,
            user = config.connection.user,
            port = config.connection.port,
            database = config.connection.database,
            password = config.connection.password
        );
        let client = Client::connect(&params, NoTls)
            .or_else(|_| Err(Error::new("Error connecting to database.")))?;
        Ok(Codegen { client, config })
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

    pub fn get_schema_file_path(&self) -> PathBuf {
        let current_dir = env::current_dir().unwrap();
        current_dir.join(&self.config.schema)
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
