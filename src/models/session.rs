use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ==========================================
// MODELS: SESSION, QUESTION, OPTION
// ==========================================

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Session {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "Avaliação Final de Matemática")]
    pub title: String,
    #[schema(example = 1)]
    pub teacher_id: i64,
    #[schema(example = 1)]
    pub class_id: i64,
    #[schema(example = 1)]
    pub subject_id: i64,
    #[schema(example = "draft")]
    pub status: String,
    #[schema(example = 600)]
    pub time_limit_seconds: i64,
    #[schema(example = "2026-03-26 01:00:00")]
    pub created_at: String,
}

// --- PAYLOAD FOR CREATING AN EMPTY SESSION ---
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSessionPayload {
    #[schema(example = "Avaliação Final de Matemática")]
    pub title: String,
    #[schema(example = 1)]
    pub teacher_id: i64,
    #[schema(example = 1)]
    pub class_id: i64,
    #[schema(example = 1)]
    pub subject_id: i64,
    #[schema(example = "draft")]
    pub status: String,
    #[schema(example = 600)]
    pub time_limit_seconds: i64,
}

// --- PAYLOADS FOR ADDING A QUESTION LATER ---
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOptionPayload {
    #[schema(example = "56")]
    pub text: String,
    #[schema(example = true)]
    pub is_correct: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateQuestionPayload {
    #[schema(example = "Quanto é 7 x 8?")]
    pub text: String,

    // A question must always be created with its options!
    pub options: Vec<CreateOptionPayload>,
}
