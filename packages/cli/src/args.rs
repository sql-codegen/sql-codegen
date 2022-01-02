pub use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Args {
    pub fn new() -> Args {
        Args::parse()
    }
}
