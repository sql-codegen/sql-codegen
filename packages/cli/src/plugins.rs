pub mod typescript;

use crate::schema;

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn run(&self, database: &schema::Database) -> String;
}
