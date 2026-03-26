use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ==========================================
// MODELS: STUDENT
// ==========================================

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Student {
    #[schema(example = 1)]
    pub id: i64,

    #[schema(example = "06.0842.2024")]
    pub student_code: String,

    #[schema(example = "Ana Langa")]
    pub name: String,

    #[schema(example = 1)]
    pub class_id: i64,

    #[schema(example = "2026-03-25 10:00:00")]
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateStudentPayload {
    #[schema(example = "06.0842.2024")]
    pub student_code: String,

    #[schema(example = "Ana Langa")]
    pub name: String,

    #[schema(example = 1)]
    pub class_id: i64,
}
