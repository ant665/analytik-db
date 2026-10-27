use clap::Parser;

mod cli;
mod errors;
mod operations;
mod utils;

use cli::{AnalytikCli, Commands};


fn main() {
    let cli = AnalytikCli::parse();
    match &cli.command {
        Commands::Schema => operations::schema(None),
        Commands::Filter(args) => { let _ = operations::filter(None, &args); },
        Commands::Select(args) => { let _ = operations::projection(None, &args); },
        Commands::Stats(args) => { let _ = operations::stats(None, &args); },
    }
}
