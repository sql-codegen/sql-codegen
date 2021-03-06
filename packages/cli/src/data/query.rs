use crate::{data, error};
use sqlparser::ast::{SetExpr, Statement};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Query<'a> {
    pub ddl: String,
    pub path: PathBuf,
    pub projection: data::Projection<'a>,
}

impl<'a> Query<'a> {
    pub fn new(path: PathBuf, ddl: String, projection: data::Projection) -> Query {
        Query {
            ddl,
            path,
            projection,
        }
    }

    pub fn from_query_file_paths(
        database: &'a data::Database,
        query_file_paths: Vec<PathBuf>,
    ) -> Result<Vec<Query<'a>>, error::CodegenError> {
        let dialect = PostgreSqlDialect {};
        let mut queries: Vec<Query> = vec![];
        for query_file_path in query_file_paths {
            let query_ddl = fs::read_to_string(&query_file_path)?;
            let query_ast = Parser::parse_sql(&dialect, &query_ddl)?;
            queries.push(Query::from_ast(
                database,
                query_file_path,
                query_ddl,
                &query_ast,
            )?);
        }
        Ok(queries)
    }

    fn from_ast(
        database: &'a data::Database,
        path: PathBuf,
        ddl: String,
        ast: &Vec<Statement>,
    ) -> Result<Query<'a>, error::CodegenError> {
        for statement in ast {
            if let Statement::Query(query) = statement {
                if let SetExpr::Select(select) = &query.body {
                    let mut projection =
                        data::Projection::from_tables_with_joins(database, &select.from)?;
                    projection.filter_by_select_items(&select.projection)?;
                    return Ok(Query::new(path, ddl, projection));
                }
            }
        }
        let path = path.to_str().unwrap();
        Err(error::CodegenError::QueryError(format!(
            "The \"{path}\" file does not contain the SELECT statement"
        )))
    }
}
