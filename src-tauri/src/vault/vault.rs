use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::Connection;

use super::backup::{create_backup, restore_backup};
use super::crypto::KEY_SIZE;
use super::db::open_db;
use super::encrypted_db::{decrypt_db, encrypt_db};
use super::key::load_or_create_key;
use super::paths::VaultPaths;

pub struct Vault {
    pub conn: Mutex<Connection>,
    pub key: [u8; KEY_SIZE],
    paths: VaultPaths,
}

impl Vault {
    pub fn new(app_data_dir: PathBuf) -> Result<Self, String> {
        let paths = VaultPaths::new(app_data_dir);

        fs::create_dir_all(&paths.root_dir)
            .map_err(|e| format!("Failed to create vault dir: {}", e))?;

        let key = load_or_create_key(&paths.root_dir.join("key.bin"))?;

        // 🔓 decrypt encrypted DB → temp
        decrypt_db(&key, &paths.enc_db_path, &paths.tmp_db_path)?;

        let conn = open_db(&paths.tmp_db_path)?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                note_type TEXT NOT NULL,
                links TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            "#,
        )
        .map_err(|e| format!("Schema init failed: {}", e))?;

        Ok(Self {
            conn: Mutex::new(conn),
            key,
            paths,
        })
    }

    /// Flush SQLite + encrypt temp DB
    pub fn seal(&self) -> Result<(), String> {
        if let Ok(conn) = self.conn.lock() {
            let _ = conn.execute_batch("PRAGMA wal_checkpoint(FULL);");
        }

        encrypt_db(
            &self.key,
            &self.paths.tmp_db_path,
            &self.paths.enc_db_path,
        )
    }

    /// Create an encrypted backup file
    pub fn backup<P: AsRef<Path>>(&self, backup_path: P) -> Result<(), String> {
        // Ensure latest state is encrypted
        self.seal()?;

        create_backup(
            &self.key,
            &self.paths.enc_db_path,
            backup_path.as_ref(),
        )
    }

    /// Restore from an encrypted backup file
    /// NOTE: App should be restarted after restore
    pub fn restore<P: AsRef<Path>>(&self, backup_path: P) -> Result<(), String> {
        restore_backup(
            &self.key,
            backup_path.as_ref(),
            &self.paths.enc_db_path,
        )
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        let _ = self.seal();
    }
}
