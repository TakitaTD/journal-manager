use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JournalEntry {
    pub title: String,
    pub content: String,
    pub encrypted: bool,
    pub created: String,
    pub updated: String,
}
impl JournalEntry {
    pub fn new(title: String, content: String, encrypted: bool) -> JournalEntry {
        return JournalEntry {
            title: title,
            content: content,
            encrypted: encrypted,
            created: Utc::now().to_rfc2822(),
            updated: Utc::now().to_rfc2822(),
        };
    }
}
