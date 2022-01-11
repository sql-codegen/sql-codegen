use crate::projection::Projection;
use sqlparser::ast::Statement;

pub struct Query {
    pub name: String,
    // pub projections: Vec<Projection>,
}

impl Query {
    pub fn new(name: String) -> Query {
        Query { name }
    }

    pub fn from_ast(name: &str, _ast: &Vec<Statement>) -> Query {
        Query::new(name.to_string())
    }
}
