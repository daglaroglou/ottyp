use totp_rs::{Algorithm, Builder, Secret, Totp};

pub fn generate_code(
    secret_base32: &str,
    algorithm: Algorithm,
    digits: u8,
    secs: u64,
) -> Result<String, String> {
    let secret = Secret::try_from_base32(secret_base32)
        .map_err(|e| format!("Invalid Base32 secret: {}", e))?;

    let totp: Totp = Builder::new()
        .with_secret(secret)
        .with_algorithm(algorithm)
        .with_digits(digits)
        .with_skew(1)
        .with_step_duration(secs)
        .build()
        .map_err(|e| format!("Failed to configure TOTP: {}", e))?;

    let code = totp.generate_current().to_string();

    Ok(code)
}
