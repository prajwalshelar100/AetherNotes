use tauri::State;

use crate::notes::{Note, NotesRepository}; // 👈 IMPORT TRAIT
use crate::vault::Vault;
use crate::vault::notes_repo::SqliteNotesRepository;

#[tauri::command]
pub fn list_notes(vault: State<Vault>) -> Result<Vec<Note>, String> {
    let conn = vault
        .conn
        .lock()
        .map_err(|_| "Failed to lock database connection")?;

    let repo = SqliteNotesRepository::new(&conn);
    repo.list_notes()
}

#[tauri::command]
pub fn create_note() -> Result<(), String> {
    Err("create_note not implemented yet".into())
}
