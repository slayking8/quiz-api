use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ==========================================
// MODELS: CLASS
// ==========================================

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Class {
    #[schema(example = 1)]
    pub id: i64,

    #[schema(example = "10ª Classe A")]
    pub name: String,

    #[schema(example = "2026-03-25 10:00:00")]
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateClassPayload {
    #[schema(example = "10ª Classe A")]
    pub name: String,
}
