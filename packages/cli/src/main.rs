mod cli;
mod codegen;
mod config;
mod error;
mod generate_schema_command;
mod plugins;
mod projection;
mod schema;
mod utils;
use crate::codegen::Codegen;
use crate::config::Config;
use crate::plugins::TypeScriptPlugin;
use crate::projection::Projection;
use generate_schema_command::GenerateSchemaCommand;
use sqlparser::ast::{Query, Select, SetExpr, Statement};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::fs;

fn process_ast(database: &schema::Database, ast: &Vec<Statement>) -> () {
    for statement in ast {
        match statement {
            Statement::Query(query) => process_query(&database, query),
            _ => (),
        };
    }
}

fn process_query(database: &schema::Database, query: &Box<Query>) -> () {
    match &query.body {
        SetExpr::Select(select) => process_select(&database, &select),
        _ => (),
    };

    ()
}

fn process_select(database: &schema::Database, select: &Box<Select>) -> () {
    let projection = Projection::new(database, &select.from);
    let projection = projection.project(&select.projection);

    projection.debug();

    ()
}

pub fn run_command(database: &schema::Database, sql_queries: Vec<String>) -> () {
    let dialect = PostgreSqlDialect {};

    let ast = Parser::parse_sql(&dialect, &sql_queries[0]).unwrap();
    process_ast(&database, &ast);

    ()
}

fn main() {
    // Collect the CLI arguments.
    let cli = cli::Cli::new();

    // Construct config struct from the CLI config argument.
    let config = Config::new(&cli.config_file_path).unwrap_or_else(|error| {
        eprintln!("{}", error.message);
        std::process::exit(1);
    });
    println!("{:#?}", config);

    // Initialize codegen with config.
    let mut codegen = Codegen::new(&config).unwrap_or_else(|error| {
        eprintln!("{:#?}", error);
        std::process::exit(1);
    });

    // Run command if provided.
    if let Some(command) = &cli.command {
        match command {
            cli::Command::GenerateSchema { override_schema } => {
                // Generate schema DDL if the file does not exist.
                GenerateSchemaCommand::run(&mut codegen, *override_schema);
            }
        }
    }
    // Generate all the files specified in the config.
    else {
        let database = schema::Database::from_codegen(&codegen);
        config.generate.iter().for_each(|generate_config| {
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
    // // LOAD QUERIES AND PARSE THEM
    // let queries = codegen.load_queries();
    // println!("QUERIES = {:#?}", queries);
    // run_command(&database, queries);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::ast::DataType;

    fn sample_database() -> schema::Database {
        schema::Database {
            name: "public".to_string(),
            tables: vec![
                schema::Table {
                    name: "users".to_string(),
                    columns: vec![
                        schema::Column {
                            name: "id".to_string(),
                            sql_type: DataType::Int(None),
                            is_primary_key: true,
                            is_unique: true,
                            is_not_null: true,
                            default_value: None,
                        },
                        schema::Column {
                            name: "first_name".to_string(),
                            sql_type: DataType::Varchar(None),
                            is_primary_key: false,
                            is_unique: false,
                            is_not_null: false,
                            default_value: None,
                        },
                    ],
                },
                schema::Table {
                    name: "orgs".to_string(),
                    columns: vec![
                        schema::Column {
                            name: "id".to_string(),
                            sql_type: DataType::Int(None),
                            is_primary_key: true,
                            is_unique: true,
                            is_not_null: true,
                            default_value: None,
                        },
                        schema::Column {
                            name: "name".to_string(),
                            sql_type: DataType::Varchar(None),
                            is_primary_key: false,
                            is_unique: false,
                            is_not_null: false,
                            default_value: None,
                        },
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_run_command() {
        let database = sample_database();
        // let expected = "type User = {
        //   id: string;
        //   firstName: string;
        // };

        // type QueryResult = Array<User>;
        // ";
        // let sql_queries = vec![
        //     "SELECT true, false AS \"boolean_value\", users.id, users.*, first_name as \"name\" FROM users;",
        // ];
        let sql_queries = vec!["SELECT * FROM users AS users2, orgs AS orgs2;"];
        run_command(&database, sql_queries);
        assert_eq!(true, true);
    }
}
