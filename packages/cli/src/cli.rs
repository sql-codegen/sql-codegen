use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub struct Cli {
    #[clap(
        help = "Provide config file path",
        long = "config",
        short = 'c',
        value_name = "PATH"
    )]
    pub config_file_path: Option<String>,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(name = "schema")]
    GenerateSchema {
        #[clap(long = "override")]
        override_schema: bool,
    },
}
