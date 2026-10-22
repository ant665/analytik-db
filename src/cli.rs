use clap::{Parser};

#[derive(Parser)]
#[command(name = "AnalytiK DB")]
#[command(about = "Gives you details about your data", long_about = None)]
pub struct Cli {
    #[arg(value_parser = is_valid_command)]
    pub command: String,
}

fn is_valid_command(s: &str) -> Result<String, String> {
    match s {
        "demo-db" => Ok("demo-db".to_string()),
        _ => Err("Unknown command".to_string())
    }
}

