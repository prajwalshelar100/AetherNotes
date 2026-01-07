use std::path::PathBuf;

pub struct VaultPaths {
    pub root_dir: PathBuf,
    pub db_path: PathBuf,
}

impl VaultPaths {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let root_dir = app_data_dir.join("vault");
        let db_path = root_dir.join("aethernotes.db");

        Self {
            root_dir,
            db_path,
        }
    }
}
