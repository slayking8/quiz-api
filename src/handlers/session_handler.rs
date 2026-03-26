use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::session::{CreateQuestionPayload, CreateSessionPayload, Session};
use crate::repositories::session_repo;

// ==========================================
// HANDLERS: SESSION
// ==========================================

#[utoipa::path(
    post,
    path = "/api/sessions",
    request_body = CreateSessionPayload,
    responses(
        (status = 201, description = "Empty session created successfully"),
        (status = 500, description = "Database constraint error")
    ),
    tag = "Sessions"
)]
pub async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateSessionPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match session_repo::create_session(&state.db_pool, &payload).await {
        Ok(id) => {
            let response = json!({
                "id": id,
                "message": "Empty session created successfully"
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
    post,
    path = "/api/sessions/{session_id}/questions",
    request_body = CreateQuestionPayload,
    params(
        ("session_id" = i64, Path, description = "The ID of the session to add the question to", example = 1)
    ),
    responses(
        (status = 201, description = "Question and its options added successfully"),
        (status = 500, description = "Database constraint error (e.g., session not found)")
    ),
    tag = "Sessions"
)]
pub async fn add_question_to_session(
    State(state): State<AppState>,
    Path(session_id): Path<i64>,
    Json(payload): Json<CreateQuestionPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match session_repo::add_question_to_session(&state.db_pool, session_id, &payload).await {
        Ok(id) => {
            let response = json!({
                "id": id,
                "message": "Question added to session successfully"
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
    path = "/api/sessions",
    responses(
        (status = 200, description = "List of all sessions", body =[Session])
    ),
    tag = "Sessions"
)]
pub async fn get_sessions(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Session>>), (StatusCode, Json<Value>)> {
    match session_repo::get_all_sessions(&state.db_pool).await {
        Ok(sessions) => Ok((StatusCode::OK, Json(sessions))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get, path = "/api/sessions/{id}",
    params(("id" = i64, Path, description = "Session ID")),
    responses((status = 200, description = "Session found", body = Session), (status = 404, description = "Not found")),
    tag = "Sessions"
)]
pub async fn get_session_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match session_repo::get_session_by_id(&state.db_pool, id).await {
        Ok(Some(session_obj)) => Ok((StatusCode::OK, Json(json!(session_obj)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Session not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[utoipa::path(
    delete, path = "/api/sessions/{id}",
    params(("id" = i64, Path, description = "Session ID")),
    responses((status = 200, description = "Deleted successfully"), (status = 404, description = "Not found")),
    tag = "Sessions"
)]
pub async fn delete_session(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match session_repo::delete_session(&state.db_pool, id).await {
        Ok(0) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Session not found" })),
        )),
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({ "message": "Session and all its questions deleted successfully" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
