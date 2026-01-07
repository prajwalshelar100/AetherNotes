use std::path::PathBuf;

pub struct VaultPaths {
    pub root_dir: PathBuf,
    pub enc_db_path: PathBuf,
    pub tmp_db_path: PathBuf,
}

impl VaultPaths {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let root_dir = app_data_dir.join("vault");

        Self {
            root_dir: root_dir.clone(),
            enc_db_path: root_dir.join("vault.db.enc"),
            tmp_db_path: root_dir.join("vault.db.tmp"),
        }
    }
}
