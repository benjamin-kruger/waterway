use clap::{Parser, Subcommand};

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
    #[command(alias = "c", about = "Create a new branch")]
    Create {
        #[arg(short)]
        message: String,
    },
    #[command(alias = "m", about = "Modify an existing branch")]
    Modify,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Create { message } => {
            println!("{:?}", message);
        }
        Commands::Modify => {
            println!("modifying");
        }
    }
}
