use clap::Parser;

mod cli;
use cli::{AnalytikCli, Commands};

mod command;
use command::{filter, schema};

fn main() {
    let cli = AnalytikCli::parse();
    match &cli.command {
        Commands::Schema => schema(None),
        Commands::Filter(args) => { filter(None, &args); },
    }
}
