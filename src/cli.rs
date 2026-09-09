use clap::{Parser, Subcommand};
use totp_rs::Algorithm;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Get {
        name: String,
    },
    Add {
        name: String,
        secret: String,
        #[arg(value_parser = parse_algorithm)]
        algorithm: Algorithm,
        digits: usize,
        step: usize,
    },
    Rm {
        name: String,
    },
    List,
}

fn parse_algorithm(s: &str) -> Result<Algorithm, String> {
    match s.to_uppercase().as_str() {
        "SHA1" | "SHA-1" => Ok(Algorithm::SHA1),
        "SHA256" | "SHA-256" => Ok(Algorithm::SHA256),
        "SHA512" | "SHA-512" => Ok(Algorithm::SHA512),
        _ => Err(format!(
            "Invalid algorithm: '{}'. Must be one of SHA1, SHA256, SHA512",
            s
        )),
    }
}
