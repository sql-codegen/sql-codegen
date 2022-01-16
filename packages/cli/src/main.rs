mod cli;
mod codegen;
mod config;
mod data;
mod error;
mod generate_schema_command;
mod plugins;
mod projection;
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
