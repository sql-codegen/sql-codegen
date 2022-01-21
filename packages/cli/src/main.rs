mod cli;
mod codegen;
mod config;
mod data;
mod duplicated_identifier;
mod error;
mod generate_schema_command;
mod plugins;
mod utils;

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
}
