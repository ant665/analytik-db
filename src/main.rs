use clap::Parser;

mod cli;
mod errors;
mod operations;
mod utils;

use cli::{AnalytikCli, Commands};

use crate::errors::AnalytikError;


fn main() -> Result<(), AnalytikError>{
    let cli = AnalytikCli::parse();
    match &cli.command {
        Commands::Schema => operations::schema(None)?,
        Commands::Filter(args) => operations::filter(None, &args)?,
        Commands::Select(args) => operations::projection(None, &args)?,
        Commands::Stats(args) => operations::stats(None, &args)?,
    };
    Ok(())
}
