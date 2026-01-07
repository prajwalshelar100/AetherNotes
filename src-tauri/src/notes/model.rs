use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,

    // fixed for now, but future-proofed
    #[serde(rename = "type")]
    pub note_type: String,

    pub links: Vec<String>,
    pub status: NoteStatus,

    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteStatus {
    Inbox,
    Active,
    Archived,
}
impl NoteStatus {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "inbox" => Ok(NoteStatus::Inbox),
            "active" => Ok(NoteStatus::Active),
            "archived" => Ok(NoteStatus::Archived),
            _ => Err(format!("Invalid note status: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            NoteStatus::Inbox => "inbox",
            NoteStatus::Active => "active",
            NoteStatus::Archived => "archived",
        }
    }
}