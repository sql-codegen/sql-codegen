mod cli;
mod codegen;
mod config;
mod data;
mod error;
mod generate_schema_command;
mod plugins;
mod projection;
mod utils;
mod x;

use crate::codegen::Codegen;

fn main() {
    // Initialize codegen with config.
    let codegen = Codegen::new().unwrap_or_else(|error| {
        eprintln!("{:#?}", error);
        std::process::exit(1);
    });
    codegen.run().unwrap_or_else(|error| {
        eprintln!("{:#?}", error);
        std::process::exit(1);
    });

    // // LOAD QUERIES AND PARSE THEM
    // let queries = codegen.load_queries();
    // println!("QUERIES = {:#?}", queries);
    // run_command(&database, queries);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::ast::DataType;

    fn sample_database() -> data::Database {
        data::Database {
            name: "public".to_string(),
            tables: vec![
                data::Table {
                    name: "users".to_string(),
                    columns: vec![
                        data::Column {
                            name: "id".to_string(),
                            sql_type: DataType::Int(None),
                            is_primary_key: true,
                            is_unique: true,
                            is_not_null: true,
                            default_value: None,
                        },
                        data::Column {
                            name: "first_name".to_string(),
                            sql_type: DataType::Varchar(None),
                            is_primary_key: false,
                            is_unique: false,
                            is_not_null: false,
                            default_value: None,
                        },
                    ],
                },
                data::Table {
                    name: "orgs".to_string(),
                    columns: vec![
                        data::Column {
                            name: "id".to_string(),
                            sql_type: DataType::Int(None),
                            is_primary_key: true,
                            is_unique: true,
                            is_not_null: true,
                            default_value: None,
                        },
                        data::Column {
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
        // let database = sample_database();
        // let expected = "type User = {
        //   id: string;
        //   firstName: string;
        // };

        // type QueryResult = Array<User>;
        // ";
        // let sql_queries = vec![
        //     "SELECT true, false AS \"boolean_value\", users.id, users.*, first_name as \"name\" FROM users;",
        // ];
        // let sql_queries = vec!["SELECT * FROM users AS users2, orgs AS orgs2;"];
        // run_command(&database, sql_queries);
        // assert_eq!(true, true);
    }
}
