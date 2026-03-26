use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ==========================================
// MODELS: TEACHER
// ==========================================

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Teacher {
    #[schema(example = 1)]
    pub id: i64,

    #[schema(example = "Carlos Mondlane")]
    pub name: String,

    #[schema(example = "carlos@escola.mz")]
    pub email: Option<String>,

    #[schema(example = "2026-03-25 10:00:00")]
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTeacherPayload {
    #[schema(example = "Carlos Mondlane")]
    pub name: String,

    #[schema(example = "carlos@escola.mz")]
    pub email: Option<String>,
}
