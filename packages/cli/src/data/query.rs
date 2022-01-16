use crate::projection::Projection;
use crate::{data, error::CodegenError};
use sqlparser::ast::{SetExpr, Statement};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Query<'a> {
    pub path: PathBuf,
    pub projections: Vec<Projection<'a>>,
}

impl<'a> Query<'a> {
    pub fn new(path: PathBuf, projections: Vec<Projection>) -> Query {
        Query { path, projections }
    }

    pub fn from_query_file_paths(
        database: &'a data::Database,
        query_file_paths: Vec<PathBuf>,
    ) -> Result<Vec<Query<'a>>, CodegenError> {
        let dialect = PostgreSqlDialect {};
        let mut queries: Vec<Query> = vec![];
        for query_file_path in query_file_paths {
            let query_ddl = fs::read_to_string(&query_file_path)?;
            let query_ast = Parser::parse_sql(&dialect, &query_ddl)?;
            queries.push(Query::from_ast(database, query_file_path, &query_ast)?);
        }
        Ok(queries)
    }

    fn from_ast(
        database: &'a data::Database,
        path: PathBuf,
        ast: &Vec<Statement>,
    ) -> Result<Query<'a>, CodegenError> {
        for statement in ast {
            if let Statement::Query(query) = statement {
                if let SetExpr::Select(select) = &query.body {
                    let projections = Projection::from(database, &select.from, &select.projection);
                    return Ok(Query::new(path, projections));
                }
            }
        }
        Err(CodegenError::QueryError(
            "Query does not contain the select statement".to_string(),
        ))
    }
}
