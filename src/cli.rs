use clap::{Parser, Subcommand, Args};

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
    Filter(FilterArgs),
    Select(SelectArgs),
    Stats(StatsArgs),
}

#[derive(Debug, Args)]
pub struct FilterArgs {
    #[arg(short, long)]
    pub column: String,
    #[arg(short, long)]
    pub gt: i64,
}

#[derive(Debug, Args)]
pub struct SelectArgs {
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub columns: Vec<String>,
}

#[derive(Debug, Args)]
pub struct StatsArgs {
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    pub column: String,
}