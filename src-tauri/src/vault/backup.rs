use std::fs;
use std::path::Path;

use super::crypto::{decrypt, encrypt, KEY_SIZE};

/// Create an encrypted backup from the encrypted DB
pub fn create_backup(
    key: &[u8; KEY_SIZE],
    enc_db_path: &Path,
    backup_path: &Path,
) -> Result<(), String> {
    if !enc_db_path.exists() {
        return Err("Encrypted database does not exist".into());
    }

    // Read encrypted DB
    let encrypted_db = fs::read(enc_db_path)
        .map_err(|e| format!("Failed to read encrypted db: {}", e))?;

    // Decrypt → re-encrypt (fresh nonce)
    let plaintext = decrypt(key, &encrypted_db)?;
    let backup_data = encrypt(key, &plaintext)?;

    fs::write(backup_path, backup_data)
        .map_err(|e| format!("Failed to write backup file: {}", e))?;

    Ok(())
}

/// Restore encrypted DB from a backup file
pub fn restore_backup(
    key: &[u8; KEY_SIZE],
    backup_path: &Path,
    enc_db_path: &Path,
) -> Result<(), String> {
    if !backup_path.exists() {
        return Err("Backup file does not exist".into());
    }

    let backup_data = fs::read(backup_path)
        .map_err(|e| format!("Failed to read backup file: {}", e))?;

    // Validate by decrypting
    let plaintext = decrypt(key, &backup_data)?;

    // Re-encrypt to standard encrypted DB format
    let encrypted_db = encrypt(key, &plaintext)?;

    fs::write(enc_db_path, encrypted_db)
        .map_err(|e| format!("Failed to restore encrypted db: {}", e))?;

    Ok(())
}
