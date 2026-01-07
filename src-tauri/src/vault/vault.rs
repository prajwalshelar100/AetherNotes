use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;

use super::db::open_db;
use super::paths::VaultPaths;

pub struct Vault {
    pub conn: Mutex<Connection>,
}

impl Vault {
    pub fn new(app_data_dir: PathBuf) -> Result<Self, String> {
        let paths = VaultPaths::new(app_data_dir);

        fs::create_dir_all(&paths.root_dir)
            .map_err(|e| format!("Failed to create vault directory: {}", e))?;

        let conn = open_db(&paths.db_path)?;

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
        .map_err(|e| format!("Failed to initialize schema: {}", e))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}
