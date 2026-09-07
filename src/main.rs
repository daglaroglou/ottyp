use clap::{Parser, Subcommand};
use totp_rs::Algorithm;

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
            todo!("Implement get function");
        }
        Commands::Add { name, secret } => {
            todo!("Implement add function")
        }
        Commands::Rm { name } => {
            todo!("Implement remove function")
        }
        Commands::List => {
            todo!("Implement list function");
        }
    }
}
