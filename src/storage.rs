use keyring::Entry;
use crate::totp;
use totp_rs::Algorithm;

const SERVICE_NAME: &str = "ottyp";
const ACCOUNT_INDEX: &str = "__accounts";

pub fn save_secret(account_name: &str, secret: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, account_name)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    if entry.get_password().is_ok() {
        return Err(format!(
            "An account named '{}' already exists.",
            account_name
        ));
    }

    entry
        .set_password(secret)
        .map_err(|e| format!("Failed to save secret: {}", e))?;

    let mut accounts = load_account_names()?;
    accounts.push(account_name.to_owned());
    save_account_names(&accounts)?;

    Ok(())
}

pub fn get_secret(account_name: &str) -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, account_name)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    if entry.get_password().is_err() {
        return Err(format!(
            "No account named '{}' was found. Please create one first.",
            account_name
        ));
    }

    entry
        .get_password()
        .map_err(|e| format!("Secret not found for {}: {}", account_name, e))
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

pub fn list_secrets() -> Result<Vec<(String, String)>, String> {
    let mut results = Vec::new();
    for name in load_account_names()? {
        let secret = get_secret(&name)?;
        let code = totp::generate_code(&secret, Algorithm::SHA1, 6, 30)?;
        results.push((name, code));
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
