use crate::cli;
use crate::config;
use crate::data;
use crate::error;
use crate::generate_schema_command::GenerateSchemaCommand;
use crate::plugins::PluginResult;
use crate::plugins::TypeScriptPgPlugin;
use crate::plugins::{
    Plugin, TypeScriptGenericSdkPlugin, TypeScriptOperationsPlugin, TypeScriptPlugin,
};
use glob::glob;
use postgres::NoTls;
use std::vec;
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
        let host = &self.config.connection.host;
        let user = &self.config.connection.user;
        let port = &self.config.connection.port;
        let database = &self.config.connection.database;
        let password = &self.config.connection.password;
        let password = match password {
            Some(password) => format!(" password={password}"),
            None => "".to_string(),
        };
        let params = format!("host={host} user={user} port={port} dbname={database}{password}");
        let client = postgres::Client::connect(&params, NoTls)?;
        Ok(client)
    }

    pub fn get_schema_file_path(&self) -> Result<PathBuf, error::CodegenError> {
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(&self.config.schema))
    }

    pub fn get_query_file_paths(&self) -> Result<Vec<PathBuf>, error::CodegenError> {
        let glob_pattern = &self.config.queries;
        let mut query_file_paths = vec![];
        let glob_result = glob(glob_pattern);
        let entries = match glob_result {
            Ok(entries) => entries,
            Err(_) => {
                return Err(error::CodegenError::ConfigError(
                    format!("Invalid glob pattern value ({glob_pattern}) in the \"queries\" config property")
                ));
            }
        };
        for entry in entries {
            match entry {
                Ok(path) => query_file_paths.push(path),
                Err(_) => {
                    return Err(error::CodegenError::ConfigError(
                        format!("Invalid glob pattern value ({glob_pattern}) in the \"queries\" config property")
                    ));
                }
            }
        }

        for query_file_path in &query_file_paths {
            let path = query_file_path.to_str().unwrap();
            match query_file_path.extension() {
                Some(extension) => {
                    if extension != "sql" {
                        return Err(error::CodegenError::ConfigError(
                            format!("Invalid glob pattern ({glob_pattern}) in the \"queries\" property that resolves to the \"{path}\" path that is not a valid SQL file"),
                        ));
                    }
                }
                None => {
                    return Err(error::CodegenError::ConfigError(
                        format!("Invalid glob pattern ({glob_pattern}) in the \"queries\" property that resolves to the \"{path}\" path that is not a valid SQL file"),
                    ));
                }
            }
        }
        Ok(query_file_paths)
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
            // Generate data to be shared between plugins, to generate types.
            let database = data::Database::from_schema_file_path(self.get_schema_file_path()?)?;
            let queries =
                data::Query::from_query_file_paths(&database, self.get_query_file_paths()?)?;
            let data = data::Data::new(&database, &queries);

            // Initialize plugins.
            let typescript_plugin = TypeScriptPlugin::new();
            let typescript_operation_plugin = TypeScriptOperationsPlugin::new(&typescript_plugin);
            let typescript_generic_sdk_plugin =
                TypeScriptGenericSdkPlugin::new(&typescript_operation_plugin);
            let typescript_pg_plugin = TypeScriptPgPlugin::new();
            let plugins: Vec<&dyn Plugin> = vec![
                &typescript_plugin,
                &typescript_operation_plugin,
                &typescript_generic_sdk_plugin,
                &typescript_pg_plugin,
            ];

            for generate_config in self.config.generate.iter() {
                let mut plugin_results: PluginResult = PluginResult::new();
                for plugin_config in generate_config.plugins.iter() {
                    let plugin = plugins
                        .iter()
                        .find(|plugin| plugin.name() == plugin_config.name);
                    if plugin.is_none() {
                        return Err(error::CodegenError::ConfigError(format!(
                            "Plugin \"{}\" not found",
                            plugin_config.name.clone()
                        )));
                    }
                    plugin_results.append(&mut plugin.unwrap().run(&data));
                }
                fs::write(&generate_config.output, plugin_results.to_string())?
            }
        }
        Ok(())
    }
}
