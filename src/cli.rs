use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "analytik")]
#[command(bin_name = "analytik")]
#[command(about = "Gives you details about your data", long_about = None)]
pub struct AnalytikCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Schema,
}