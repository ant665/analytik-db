use clap::Parser;

mod cli;
use cli::Cli;

mod command;
use command::schema;

fn main() {
    let _cli = Cli::parse();
    schema(None);
}
