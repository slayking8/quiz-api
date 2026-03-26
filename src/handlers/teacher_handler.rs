use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::teacher::{CreateTeacherPayload, Teacher};
use crate::repositories::teacher_repo;

#[utoipa::path(
    post,
    path = "/api/teachers",
    request_body = CreateTeacherPayload,
    responses(
        (status = 201, description = "Teacher created successfully"),
        (status = 500, description = "Database error or email already exists")
    ),
    tag = "Teachers"
)]
pub async fn create_teacher(
    State(state): State<AppState>,
    Json(payload): Json<CreateTeacherPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match teacher_repo::create_teacher(&state.db_pool, &payload).await {
        Ok(id) => {
            let response = json!({ "id": id, "message": "Teacher created successfully" });
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
    path = "/api/teachers",
    responses(
        (status = 200, description = "List of all teachers", body = [Teacher])
    ),
    tag = "Teachers"
)]
pub async fn get_teachers(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Teacher>>), (StatusCode, Json<Value>)> {
    match teacher_repo::get_all_teachers(&state.db_pool).await {
        Ok(teachers) => Ok((StatusCode::OK, Json(teachers))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get, path = "/api/teachers/{id}",
    params(("id" = i64, Path, description = "Teacher ID")),
    responses((status = 200, description = "Teacher found", body = Teacher), (status = 404, description = "Not found")),
    tag = "Teachers"
)]
pub async fn get_teacher_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match teacher_repo::get_teacher_by_id(&state.db_pool, id).await {
        Ok(Some(teacher)) => Ok((StatusCode::OK, Json(json!(teacher)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Teacher not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[utoipa::path(
    delete, path = "/api/teachers/{id}",
    params(("id" = i64, Path, description = "Teacher ID")),
    responses((status = 200, description = "Deleted successfully"), (status = 404, description = "Not found")),
    tag = "Teachers"
)]
pub async fn delete_teacher(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match teacher_repo::delete_teacher(&state.db_pool, id).await {
        Ok(0) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Teacher not found" })),
        )),
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({ "message": "Teacher deleted successfully" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
