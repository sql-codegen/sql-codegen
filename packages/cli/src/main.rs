mod args;
mod codegen;
mod config;
mod ddl_schema;
mod projection;
mod schema;
mod typescript;
mod utils;

use crate::args::Args;
use crate::codegen::Codegen;
use crate::config::Config;
use crate::projection::Projection;
use crate::schema::Database;
use sqlparser::ast::{DataType, Query, Select, SetExpr, Statement};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

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
    let args = Args::new();

    // CONFIG
    let config = Config::new(args);
    println!("Configuration file\n{:#?}", config);

    // GENERATE SCHEMA DDL
    let mut ddl_schema = ddl_schema::DdlSchema::new(&config);
    ddl_schema.generate();

    // typescript::types::generate(&tables);
    // INIT CODEGEN
    let codegen = Codegen::new(&config);
    // CONVERT SCHEMA DDL TO DATABASE STRUCT
    let schema_ddl = codegen.load_schema_ddl();
    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, &schema_ddl).unwrap();
    let database = Database::from_statements(&ast);
    println!("{:#?}", ast);
    println!("{:#?}", database);
    // LOAD QUERIES AND PARSE THEM
    let queries = codegen.load_queries();
    println!("QUERIES = {:#?}", queries);
    run_command(&database, queries);
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
