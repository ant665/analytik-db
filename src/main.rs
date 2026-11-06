use arrow::util::pretty::pretty_format_batches;
use clap::Parser;
use std::slice::from_ref;

mod cli;
mod errors;
mod operations;
mod storage;
mod utils;

use cli::{AnalytikCli, Commands};

use crate::errors::AnalytikError;


fn main() -> Result<(), AnalytikError> {
    let cli = AnalytikCli::parse();
    match &cli.command {
        Commands::Filter(args) => {
            operations::filter(None, &args)?;
        },
        Commands::Select(args) => {
            let projection = operations::projection(None, &args)?;
            println!("{}", pretty_format_batches(from_ref(&projection)).unwrap());
        },
        Commands::Stats(args) => {
            let stats = operations::stats(None, &args)?;
            println!("{}", stats);
        },
        Commands::Inspect(args) => {
            let schema = operations::inspect(&args)?;
            println!("{}", schema);
        },
    };
    Ok(())
}
