use crate::notes::model::Note;

pub trait NotesRepository {
    fn list_notes(&self) -> Result<Vec<Note>, String>;
    fn create_note(&self, note: Note) -> Result<(), String>;
    fn update_note(&self, note: Note) -> Result<(), String>;
}
