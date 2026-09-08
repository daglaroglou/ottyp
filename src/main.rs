use clap::{Parser, Subcommand};
use comfy_table::Table;
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
            match storage::get_secret(name) {
                Ok(secret) => {
                    match totp::generate_code(&secret, Algorithm::SHA1, 6, 30) {
                        Ok(code) => {
                            if let Err(e) = clipboard::copy_to_clipboard(&code) {
                                eprintln!("Warning: {}", e);
                            }
                            println!("Code for {}: {}", name, code);
                            println!("(Copied to clipboard)");
                        }
                        Err(err) => eprintln!("Error: {}", err),
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        Commands::Add { name, secret } => {
            match storage::save_secret(name, secret) {
                Ok(_) => println!("Successfully saved secret for {}.", name),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Commands::Rm { name } => {
            match storage::delete_secret(name) {
                Ok(_) => println!("Successfully deleted secret for {}.", name),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Commands::List => {
            match storage::list_secrets() {
                Ok(accounts) => {
                    let mut table = Table::new();
                    table.set_header(vec!["Account Name", "TOTP Code"]);
                    for (name, code) in accounts {
                        table.add_row(vec![name, code]);
                    }
                    println!("{table}");
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }
    }
}
