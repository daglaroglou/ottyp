use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod clipboard;
mod storage;
mod totp;
mod ui;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get { name } => match storage::get_secret(name) {
            Ok(account) => {
                match totp::generate_code(
                    &account.secret,
                    account.algorithm,
                    account.digits as u8,
                    account.step as u64,
                ) {
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
        },

        Commands::Add {
            name,
            secret,
            algorithm,
            digits,
            step,
        } => match storage::save_secret(name, secret, *algorithm, *digits, *step) {
            Ok(_) => println!("Successfully saved secret for {}.", name),
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::Rm { name } => match storage::delete_secret(name) {
            Ok(_) => println!("Successfully removed account: {}", name),
            Err(err) => eprintln!("Error: {}", err),
        },

        Commands::List => match storage::list_secrets() {
            Ok(accounts) => ui::print_accounts_table(accounts),
            Err(err) => eprintln!("Error: {}", err),
        },
    }
}
