use rusqlite::params;
use serde_json;

use crate::notes::{Note, NotesRepository};
use crate::notes::model::NoteStatus;

pub struct SqliteNotesRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SqliteNotesRepository<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }
}

impl<'a> NotesRepository for SqliteNotesRepository<'a> {
    fn list_notes(&self) -> Result<Vec<Note>, String> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, title, content, note_type, links, status, created_at, updated_at
                FROM notes
                ORDER BY updated_at DESC
                "#,
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let links_json: String = row.get(4)?;
                let links: Vec<String> =
                    serde_json::from_str(&links_json).unwrap_or_default();

                let status_str: String = row.get(5)?;
                let status = NoteStatus::from_str(&status_str)
                    .map_err(|e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            5,
                            rusqlite::types::Type::Text,
                            Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                e,
                            )),
                        )
                    })?;

                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    note_type: row.get(3)?,
                    links,
                    status,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut notes = Vec::new();
        for note in rows {
            notes.push(note.map_err(|e| e.to_string())?);
        }

        Ok(notes)
    }

     fn create_note(&self, note: Note) -> Result<(), String> {
        let links_json =
            serde_json::to_string(&note.links).map_err(|e| e.to_string())?;

        self.conn
            .execute(
                r#"
                INSERT INTO notes (
                    id, title, content, note_type, links, status, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                params![
                    note.id,
                    note.title,
                    note.content,
                    note.note_type,
                    links_json,
                    note.status.as_str(),
                    note.created_at,
                    note.updated_at
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }


       fn update_note(&self, note: Note) -> Result<(), String> {
        let links_json =
            serde_json::to_string(&note.links).map_err(|e| e.to_string())?;

        self.conn
            .execute(
                r#"
                UPDATE notes
                SET
                    title = ?2,
                    content = ?3,
                    note_type = ?4,
                    links = ?5,
                    status = ?6,
                    updated_at = ?7
                WHERE id = ?1
                "#,
                params![
                    note.id,
                    note.title,
                    note.content,
                    note.note_type,
                    links_json,
                    note.status.as_str(),
                    note.updated_at
                ],
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }

}
