use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use super::crypto::{generate_key, KEY_SIZE};

/// Load existing master key or create a new one
pub fn load_or_create_key(path: &Path) -> Result<[u8; KEY_SIZE], String> {
    if path.exists() {
        load_key(path)
    } else {
        create_key(path)
    }
}

fn load_key(path: &Path) -> Result<[u8; KEY_SIZE], String> {
    let mut file = fs::File::open(path)
        .map_err(|e| format!("Failed to open key file: {}", e))?;

    let mut buf = [0u8; KEY_SIZE];
    file.read_exact(&mut buf)
        .map_err(|e| format!("Failed to read key file: {}", e))?;

    Ok(buf)
}

fn create_key(path: &Path) -> Result<[u8; KEY_SIZE], String> {
    let key = generate_key();

    let mut file = fs::File::create(path)
        .map_err(|e| format!("Failed to create key file: {}", e))?;

    file.write_all(&key)
        .map_err(|e| format!("Failed to write key file: {}", e))?;

    Ok(key)
}
