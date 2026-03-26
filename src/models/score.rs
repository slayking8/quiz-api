use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// ==========================================
// MODELS: SCORE
// ==========================================

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Score {
    #[schema(example = 1)]
    pub id: i64,
    #[schema(example = 1)]
    pub session_id: i64,
    #[schema(example = 1)]
    pub student_id: i64,
    #[schema(example = 6)]
    pub score: i64,
    #[schema(example = "2026-03-25 20:30:00")]
    pub played_at: String,
    #[schema(example = "2026-03-26 08:00:00")]
    pub synced_at: String,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub local_attempt_id: String,
}

/// Represents the payload sent by the frontend during the offline sync process.
/// The frontend will send a JSON Array of these objects.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ScoreSyncPayload {
    #[schema(example = 1)]
    pub session_id: i64,
    #[schema(example = 1)]
    pub student_id: i64,
    #[schema(example = 6)]
    pub score: i64,

    // The exact time the student finished the quiz on their device
    #[schema(example = "2026-03-25 20:30:00")]
    pub played_at: String,
    // The frontend must generate a UUID for every attempt made offline
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub local_attempt_id: String,
}
