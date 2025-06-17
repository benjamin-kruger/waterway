use std::env::current_dir;

use clap::{ArgAction, Parser, Subcommand};

use waterway::commands;

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
    Init {
        #[arg(help = "The trunk branch name for this repository")]
        trunk: String,
    },
    #[command(alias = "c", about = "Create a new branch")]
    Create {
        #[arg(short)]
        message: String,
    },
    #[command(alias = "m", about = "Modify an existing branch")]
    Modify {
        #[arg(short, help = "Optionally modify the current commit message")]
        message: Option<String>,
    },
    #[command(alias = "r", about = "Restack a branch onto its parent")]
    Restack {
        #[arg(help = "The branch to restack - uses the current branch by default")]
        branch: Option<String>,
        #[arg(long = "continue", short, action = ArgAction::SetTrue, help = "Continue an existing restack operation")]
        cont: bool,
    },
}

fn main() {
    let args = Cli::parse();
    let dir = current_dir().expect("could not fetch current dir");

    match args.command {
        Commands::Init { trunk } => {
            commands::init(dir, trunk);
        }
        Commands::Create { message } => {
            commands::create(dir, message);
        }
        Commands::Modify { message } => {
            commands::modify(dir, message);
        }
        Commands::Restack { branch, cont } => {
            commands::restack(dir, branch, cont);
        }
    }
}
