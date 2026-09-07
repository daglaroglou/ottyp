use clap::{Parser, Subcommand};

mod cli;
mod clipboard;
mod storage;
mod totp;
mod ui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get {
        name: String
    },

    Add {
        name: String,
        secret: String
    },

    Rm {
        name: String
    },

    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { name } => {
            println!("Fetching code for: {}", name);
        }
        Commands::Add { name, secret } => {
            println!("Adding secret for: {} (secret: {})", name, secret);
        }
        Commands::Rm { name } => {
            println!("Removing account: {}", name);
        }
        Commands::List => {
            println!("Listing all accounts...");
        }
    }
}
