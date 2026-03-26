use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ==========================================
// MODELS: SUBJECT
// ==========================================

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Subject {
    #[schema(example = 1)]
    pub id: i64,

    #[schema(example = "Matemática")]
    pub name: String,

    #[schema(example = "2026-03-25 10:00:00")]
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubjectPayload {
    #[schema(example = "Matemática")]
    pub name: String,
}
