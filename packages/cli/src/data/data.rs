use super::{Database, Query};

pub struct Data<'a> {
    pub database: &'a Database,
    pub queries: &'a Vec<Query<'a>>,
}

impl<'a> Data<'a> {
    pub fn new(database: &'a Database, queries: &'a Vec<Query<'a>>) -> Data<'a> {
        Data { database, queries }
    }
}
