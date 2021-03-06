use crate::{
    error::{DemonlistError, Result},
    record::note::Note,
};
use pointercrate_core::util::non_nullable;
use serde::Deserialize;
use sqlx::PgConnection;

#[derive(Debug, Deserialize)]
pub struct PatchNote {
    #[serde(default, deserialize_with = "non_nullable")]
    pub content: Option<String>,
}

impl Note {
    pub async fn apply_patch(mut self, patch: PatchNote, connection: &mut PgConnection) -> Result<Note> {
        if let Some(content) = patch.content {
            if content.trim().is_empty() {
                return Err(DemonlistError::NoteEmpty)
            }

            sqlx::query!("UPDATE record_notes SET content = $1 WHERE id = $2", content, self.id)
                .execute(connection)
                .await?;

            self.content = content;
        }

        Ok(self)
    }
}
