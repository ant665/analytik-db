use clap::Parser;

mod cli;
use cli::{AnalytikCli, Commands};

mod command;
use command::schema;

fn main() {
    let cli = AnalytikCli::parse();
    match &cli.command {
        Commands::Schema => schema(None),
    }
}
