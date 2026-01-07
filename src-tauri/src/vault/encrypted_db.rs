use std::fs;
use std::path::Path;

use super::crypto::{decrypt, encrypt, KEY_SIZE};

pub fn decrypt_db(
    key: &[u8; KEY_SIZE],
    enc_path: &Path,
    tmp_path: &Path,
) -> Result<(), String> {
    if !enc_path.exists() {
        return Ok(()); // first run
    }

    let encrypted = fs::read(enc_path)
        .map_err(|e| format!("Failed to read encrypted db: {}", e))?;

    let plaintext = decrypt(key, &encrypted)?;
    fs::write(tmp_path, plaintext)
        .map_err(|e| format!("Failed to write temp db: {}", e))?;

    Ok(())
}

pub fn encrypt_db(
    key: &[u8; KEY_SIZE],
    tmp_path: &Path,
    enc_path: &Path,
) -> Result<(), String> {
    if !tmp_path.exists() {
        return Ok(());
    }

    let plaintext = fs::read(tmp_path)
        .map_err(|e| format!("Failed to read temp db: {}", e))?;

    let encrypted = encrypt(key, &plaintext)?;
    fs::write(enc_path, encrypted)
        .map_err(|e| format!("Failed to write encrypted db: {}", e))?;

    fs::remove_file(tmp_path).ok();
    Ok(())
}
