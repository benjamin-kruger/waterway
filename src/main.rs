use std::env::current_dir;

use clap::{Parser, Subcommand};
mod commands;
mod git;

#[derive(Parser, Debug)]
#[command(
    name = "waterway",
    bin_name = "ww",
    about = "Waterway is a tool for managing stacked diffs",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(alias = "i", about = "Initiliase waterway in the current repository")]
    Init,
    #[command(alias = "c", about = "Create a new branch")]
    Create {
        #[arg(short)]
        message: String,
    },
    #[command(alias = "m", about = "Modify an existing branch")]
    Modify {
        #[arg(short)]
        message: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();
    let dir = current_dir().expect("could not fetch current dir");

    match args.command {
        Commands::Init => {
            commands::init(dir);
        }
        Commands::Create { message } => {
            commands::create(dir, message);
        }
        Commands::Modify { message } => {
            commands::modify(dir, message);
        }
    }
}
