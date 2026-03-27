use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::session::{
    CreateQuestionPayload, CreateSessionPayload, QuestionResponse, Session,
    UpdateSessionStatusPayload,
};
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

#[utoipa::path(
    post,
    path = "/api/sessions/{session_id}/questions/bulk-text",
    request_body(content = String, description = "Raw text content with specific formatting", content_type = "text/plain"),
    params(
        ("session_id" = i64, Path, description = "The ID of the session")
    ),
    responses(
        (status = 201, description = "Bulk questions parsed and added atomically"),
        (status = 400, description = "Invalid text format"),
        (status = 500, description = "Database constraint error")
    ),
    tag = "Sessions"
)]
pub async fn upload_bulk_questions_text(
    State(state): State<AppState>,
    Path(session_id): Path<i64>,
    body: String, // Accepts the raw text file content
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let mut parsed_questions: Vec<CreateQuestionPayload> = Vec::new();

    // Normalize line endings (Windows \r\n to Linux \n) and split by double newlines
    let normalized_text = body.replace("\r\n", "\n");
    let blocks = normalized_text.split("\n\n");

    for block in blocks {
        let lines: Vec<&str> = block.lines().filter(|l| !l.trim().is_empty()).collect();

        if lines.is_empty() {
            continue;
        }

        // The first line of the block is the question text
        let question_text = lines[0].trim().to_string();
        let mut options: Vec<crate::models::session::CreateOptionPayload> = Vec::new();

        // The following lines are the options (* for correct, - for wrong)
        for line in lines.iter().skip(1) {
            let line = line.trim();
            if line.starts_with('*') {
                options.push(crate::models::session::CreateOptionPayload {
                    text: line[1..].trim().to_string(),
                    is_correct: true,
                });
            } else if line.starts_with('-') {
                options.push(crate::models::session::CreateOptionPayload {
                    text: line[1..].trim().to_string(),
                    is_correct: false,
                });
            }
        }

        // Only add if the question has at least one option to avoid corrupt data
        if !options.is_empty() {
            parsed_questions.push(CreateQuestionPayload {
                text: question_text,
                options,
            });
        }
    }

    if parsed_questions.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No valid questions found in the provided text." })),
        ));
    }

    // Pass the parsed array to our atomic repository function
    match session_repo::add_bulk_questions_to_session(&state.db_pool, session_id, &parsed_questions)
        .await
    {
        Ok(count) => {
            let response = json!({
                "message": "Bulk questions inserted successfully",
                "questions_added": count
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
    path = "/api/sessions/{session_id}/questions",
    params(
        ("session_id" = i64, Path, description = "The ID of the session", example = 1)
    ),
    responses(
        (status = 200, description = "List of questions with their nested options", body = [QuestionResponse]),
        (status = 500, description = "Database error")
    ),
    tag = "Sessions"
)]
pub async fn get_session_questions(
    State(state): State<AppState>,
    Path(session_id): Path<i64>,
) -> Result<(StatusCode, Json<Vec<QuestionResponse>>), (StatusCode, Json<Value>)> {
    match session_repo::get_questions_for_session(&state.db_pool, session_id).await {
        Ok(questions) => Ok((StatusCode::OK, Json(questions))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    patch,
    path = "/api/sessions/{session_id}/status",
    request_body = UpdateSessionStatusPayload,
    params(
        ("session_id" = i64, Path, description = "The ID of the session")
    ),
    responses(
        (status = 200, description = "Session status updated successfully"),
        (status = 400, description = "Invalid status provided"),
        (status = 404, description = "Session not found"),
        (status = 500, description = "Database error")
    ),
    tag = "Sessions"
)]
pub async fn update_session_status(
    State(state): State<AppState>,
    Path(session_id): Path<i64>,
    Json(payload): Json<UpdateSessionStatusPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    // 1. Validate the status explicitly before hitting the database
    let valid_statuses = ["draft", "active", "completed"];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid status. Must be 'draft', 'active', or 'completed'" })),
        ));
    }

    // 2. Update the database
    match session_repo::update_session_status(&state.db_pool, session_id, &payload.status).await {
        Ok(0) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Session not found" })),
        )),
        Ok(_) => {
            let response =
                json!({ "message": format!("Session status changed to '{}'", payload.status) });
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}
