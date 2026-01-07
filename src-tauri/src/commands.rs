use tauri::State;

use crate::notes::{Note, NotesRepository};
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
pub fn create_note(vault: State<Vault>, note: Note) -> Result<(), String> {
    let conn = vault
        .conn
        .lock()
        .map_err(|_| "Failed to lock database connection")?;

    let repo = SqliteNotesRepository::new(&conn);
    repo.create_note(note)
}

#[tauri::command]
pub fn update_note(vault: State<Vault>, note: Note) -> Result<(), String> {
    let conn = vault
        .conn
        .lock()
        .map_err(|_| "Failed to lock database connection")?;

    let repo = SqliteNotesRepository::new(&conn);
    repo.update_note(note)
}



#[tauri::command]
pub fn backup_vault(vault: State<Vault>, backup_path: String) -> Result<(), String> {
    vault.backup(backup_path)
}

#[tauri::command]
pub fn restore_vault(vault: State<Vault>, backup_path: String) -> Result<(), String> {
    vault.restore(backup_path)
}
#[tauri::command]
pub fn delete_note(
    vault: tauri::State<Vault>,
    note_id: String,
) -> Result<(), String> {
    let conn = vault
        .conn
        .lock()
        .map_err(|_| "Failed to lock database connection")?;

    let repo = SqliteNotesRepository::new(&conn);
    repo.delete_note(note_id)
}
