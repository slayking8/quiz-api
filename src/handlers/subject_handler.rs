use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::subject::{CreateSubjectPayload, Subject};
use crate::repositories::subject_repo;

#[utoipa::path(
    post,
    path = "/api/subjects",
    request_body = CreateSubjectPayload,
    responses(
        (status = 201, description = "Subject created successfully"),
        (status = 500, description = "Database error")
    ),
    tag = "Subjects"
)]
pub async fn create_subject(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubjectPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match subject_repo::create_subject(&state.db_pool, &payload).await {
        Ok(id) => {
            let response = json!({ "id": id, "message": "Subject created successfully" });
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
    path = "/api/subjects",
    responses(
        (status = 200, description = "List of all subjects", body = [Subject])
    ),
    tag = "Subjects"
)]
pub async fn get_subjects(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Subject>>), (StatusCode, Json<Value>)> {
    match subject_repo::get_all_subjects(&state.db_pool).await {
        Ok(subjects) => Ok((StatusCode::OK, Json(subjects))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get, path = "/api/subjects/{id}",
    params(("id" = i64, Path, description = "Subject ID")),
    responses((status = 200, description = "Subject found", body = Subject), (status = 404, description = "Not found")),
    tag = "Subjects"
)]
pub async fn get_subject_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match subject_repo::get_subject_by_id(&state.db_pool, id).await {
        Ok(Some(subject)) => Ok((StatusCode::OK, Json(json!(subject)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Subject not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[utoipa::path(
    delete, path = "/api/subjects/{id}",
    params(("id" = i64, Path, description = "Subject ID")),
    responses((status = 200, description = "Deleted successfully"), (status = 404, description = "Not found")),
    tag = "Subjects"
)]
pub async fn delete_subject(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match subject_repo::delete_subject(&state.db_pool, id).await {
        Ok(0) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Subject not found" })),
        )),
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({ "message": "Subject deleted successfully" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
