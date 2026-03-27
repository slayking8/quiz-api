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

// MODELS FOR GETTING QUESTIONS & OPTIONS
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct OptionResponse {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "Maputo")]
    pub text: String,
    #[schema(example = true)]
    pub is_correct: bool,
}

/// A temporary struct to map the database row before attaching options
#[derive(Debug, FromRow)]
pub struct QuestionDbRow {
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QuestionResponse {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = "Qual é a capital de Moçambique?")]
    pub text: String,
    pub options: Vec<OptionResponse>,
}

/// Payload for updating the status of a session
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSessionStatusPayload {
    /// Must be 'draft', 'active', or 'completed'
    #[schema(example = "draft or active or completed")]
    pub status: String,
}
