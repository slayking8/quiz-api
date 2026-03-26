use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::score::{Score, ScoreSyncPayload};
use crate::repositories::score_repo;

// ==========================================
// HANDLERS: SCORE
// ==========================================

#[utoipa::path(
    post,
    path = "/api/scores/sync",
    request_body = [ScoreSyncPayload],
    responses(
        (status = 201, description = "Scores synchronized successfully"),
        (status = 500, description = "Database constraint error (e.g., invalid student_id)")
    ),
    tag = "Scores"
)]
pub async fn sync_scores(
    State(state): State<AppState>,
    Json(payload): Json<Vec<ScoreSyncPayload>>, // Note: We expect a JSON Array!
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match score_repo::sync_scores(&state.db_pool, &payload).await {
        Ok(count) => {
            let response = json!({
                "message": format!("Successfully synced {} scores", count),
                "synced_count": count
            });
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/sessions/{session_id}/scores",
    params(
        ("session_id" = i64, Path, description = "The ID of the session", example = 1)
    ),
    responses(
        (status = 200, description = "List of scores for the session (Leaderboard)", body = [Score])
    ),
    tag = "Scores"
)]
pub async fn get_session_scores(
    State(state): State<AppState>,
    Path(session_id): Path<i64>,
) -> Result<(StatusCode, Json<Vec<Score>>), (StatusCode, Json<Value>)> {
    match score_repo::get_scores_by_session(&state.db_pool, session_id).await {
        Ok(scores) => Ok((StatusCode::OK, Json(scores))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}
