use crate::totp;
use keyring::Entry;
use totp_rs::Algorithm;

pub struct AccountData {
    pub secret: String,
    pub algorithm: Algorithm,
    pub digits: usize,
    pub step: usize,
}

fn format_algorithm(algo: &Algorithm) -> &'static str {
    match algo {
        Algorithm::SHA1 => "SHA1",
        Algorithm::SHA256 => "SHA256",
        Algorithm::SHA512 => "SHA512",
        _ => "SHA1",
    }
}

fn parse_algorithm(s: &str) -> Result<Algorithm, String> {
    match s.to_uppercase().as_str() {
        "SHA1" | "SHA-1" => Ok(Algorithm::SHA1),
        "SHA256" | "SHA-256" => Ok(Algorithm::SHA256),
        "SHA512" | "SHA-512" => Ok(Algorithm::SHA512),
        _ => Err(format!("Invalid algorithm: {}", s)),
    }
}

const SERVICE_NAME: &str = "ottyp";
const ACCOUNT_INDEX: &str = "__accounts";

pub fn save_secret(
    account_name: &str,
    secret: &str,
    algorithm: Algorithm,
    digits: usize,
    step: usize,
) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, account_name)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    if entry.get_password().is_ok() {
        return Err(format!(
            "An account named '{}' already exists.",
            account_name
        ));
    }

    let payload = format!(
        "{}:{}:{}:{}",
        secret,
        format_algorithm(&algorithm),
        digits,
        step
    );

    entry
        .set_password(&payload)
        .map_err(|e| format!("Failed to save secret: {}", e))?;

    let mut accounts = load_account_names()?;
    accounts.push(account_name.to_owned());
    save_account_names(&accounts)?;

    Ok(())
}

pub fn get_secret(account_name: &str) -> Result<AccountData, String> {
    let entry = Entry::new(SERVICE_NAME, account_name)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    if entry.get_password().is_err() {
        return Err(format!(
            "No account named '{}' was found. Please create one first.",
            account_name
        ));
    }

    let raw = entry
        .get_password()
        .map_err(|e| format!("Secret not found for {}: {}", account_name, e))?;

    let parts: Vec<&str> = raw.split(':').collect();
    if parts.len() == 1 {
        return Ok(AccountData {
            secret: parts[0].to_string(),
            algorithm: Algorithm::SHA1,
            digits: 6,
            step: 30,
        });
    }

    if parts.len() != 4 {
        return Err(format!("Invalid data format for account {}", account_name));
    }

    let algorithm = parse_algorithm(parts[1])?;
    let digits = parts[2].parse().map_err(|_| "Invalid digits format")?;
    let step = parts[3].parse().map_err(|_| "Invalid step format")?;

    Ok(AccountData {
        secret: parts[0].to_string(),
        algorithm,
        digits,
        step,
    })
}

pub fn delete_secret(account_name: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, account_name)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    if entry.get_password().is_err() {
        return Err(format!(
            "No account named '{}' was found. Please create one first.",
            account_name
        ));
    }

    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete account '{}': {}", account_name, e))?;

    let mut accounts = load_account_names()?;
    accounts.retain(|name| name != account_name);
    save_account_names(&accounts)?;

    Ok(())
}

pub fn list_secrets() -> Result<Vec<(String, String, usize)>, String> {
    let mut results = Vec::new();
    for name in load_account_names()? {
        let account = get_secret(&name)?;
        let code = totp::generate_code(
            &account.secret,
            account.algorithm,
            account.digits as u8,
            account.step as u64,
        )?;
        results.push((name, code, account.step));
    }
    Ok(results)
}

fn load_account_names() -> Result<Vec<String>, String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_INDEX)
        .map_err(|e| format!("Failed to create account index: {}", e))?;

    match entry.get_password() {
        Ok(value) if !value.is_empty() => Ok(value.lines().map(str::to_owned).collect()),
        Ok(_) => Ok(Vec::new()),
        Err(_) => Ok(Vec::new()),
    }
}

fn save_account_names(accounts: &[String]) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_INDEX)
        .map_err(|e| format!("Failed to create account index: {}", e))?;

    entry
        .set_password(&accounts.join("\n"))
        .map_err(|e| format!("Failed to save account index: {}", e))
}
