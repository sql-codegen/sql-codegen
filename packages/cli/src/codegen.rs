use crate::cli;
use crate::config;
use crate::data;
use crate::error::CodegenError;
use crate::generate_schema_command::GenerateSchemaCommand;
use crate::plugins::{Plugin, TypeScriptOperationsPlugin, TypeScriptPlugin};
use postgres::NoTls;
use std::vec;
use std::{env, fs, path::PathBuf};

#[derive()]
pub struct Codegen {
    pub cli: cli::Cli,
    pub config: config::Config,
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl Codegen {
    pub fn new() -> Result<Codegen, CodegenError> {
        // Collect the CLI arguments.
        let cli = cli::Cli::new();

        // Create config struct from the CLI config argument.
        let config = config::Config::new(&cli.config_file_path)?;
        println!("{:#?}", config);

        Ok(Codegen {
            cli,
            config,
            plugins: vec![
                Box::new(TypeScriptPlugin::new()),
                Box::new(TypeScriptOperationsPlugin::new()),
            ],
        })
    }

    pub fn connect(&self) -> Result<postgres::Client, CodegenError> {
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

    pub fn get_queries_file_path(&self) -> PathBuf {
        let current_dir = env::current_dir().unwrap();
        current_dir.join(&self.config.queries)
    }

    pub fn run(&self) -> Result<(), CodegenError> {
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
            let data = data::Data::new(self.get_schema_file_path(), self.get_queries_file_path())?;
            for generate_config in self.config.generate.iter() {
                let mut plugins_code: Vec<String> = vec![];
                for plugin_config in generate_config.plugins.iter() {
                    let plugin = self
                        .plugins
                        .iter()
                        .find(|plugin| plugin.name() == plugin_config.name);
                    if plugin.is_none() {
                        return Err(CodegenError::PluginNotFoundError(
                            plugin_config.name.clone(),
                        ));
                    }
                    plugins_code.push(plugin.unwrap().run(&data));
                }
                fs::write(&generate_config.output, plugins_code.join("\n"))?
            }
        }
        Ok(())
    }
}
