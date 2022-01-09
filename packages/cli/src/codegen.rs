use crate::cli;
use crate::config;
use crate::error;
use crate::generate_schema_command::GenerateSchemaCommand;
use crate::plugins::TypeScriptPlugin;
use crate::schema;
use postgres::{Client, NoTls};
use std::{env, fs, path::PathBuf};

#[derive()]
pub struct Codegen {
    pub cli: cli::Cli,
    pub config: config::Config,
}

impl Codegen {
    pub fn new() -> Result<Codegen, error::CodegenError> {
        // Collect the CLI arguments.
        let cli = cli::Cli::new();

        // Create config struct from the CLI config argument.
        let config = config::Config::new(&cli.config_file_path)?;
        println!("{:#?}", config);

        Ok(Codegen { cli, config })
    }

    pub fn connect(&self) -> Result<postgres::Client, error::CodegenError> {
        let params = format!(
            "host={host} user={user} port={port} dbname={database} password={password}",
            host = self.config.connection.host,
            user = self.config.connection.user,
            port = self.config.connection.port,
            database = self.config.connection.database,
            password = self.config.connection.password
        );
        let client = postgres::Client::connect(&params, NoTls)?;
        Ok(client)
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

    pub fn run(&self) -> Result<(), error::CodegenError> {
        // Run command if provided.
        if let Some(command) = &self.cli.command {
            match command {
                cli::Command::GenerateSchema { override_schema } => {
                    // Generate schema DDL if the file does not exist.
                    GenerateSchemaCommand::run(self, *override_schema)?;
                }
            }
        }
        // Generate all the files specified in the config.
        else {
            // Create database struct from the schema file.
            let database = schema::Database::from_schema_file_path(&self.config.schema)?;

            self.config.generate.iter().for_each(|generate_config| {
                let code = generate_config
                    .plugins
                    .iter()
                    .map(|plugin_config| {
                        if plugin_config.name == "typescript" {
                            return TypeScriptPlugin::run(&database);
                        }
                        String::from("")
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                fs::write(&generate_config.output, code).expect("Error creating output file");
            });
        }
        Ok(())
    }
}
