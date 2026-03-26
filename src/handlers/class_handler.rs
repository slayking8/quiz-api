use axum::extract::{Path, State};
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::class::{Class, CreateClassPayload};
use crate::repositories::class_repo;

#[utoipa::path(
    post,
    path = "/api/classes",
    request_body = CreateClassPayload,
    responses(
        (status = 201, description = "Class created successfully"),
        (status = 500, description = "Database error or name already exists")
    ),
    tag = "Classes"
)]
pub async fn create_class(
    State(state): State<AppState>,
    Json(payload): Json<CreateClassPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match class_repo::create_class(&state.db_pool, &payload).await {
        Ok(id) => {
            let response = json!({ "id": id, "message": "Class created successfully" });
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
    path = "/api/classes",
    responses(
        (status = 200, description = "List of all classes", body = [Class])
    ),
    tag = "Classes"
)]
pub async fn get_classes(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Class>>), (StatusCode, Json<Value>)> {
    match class_repo::get_all_classes(&state.db_pool).await {
        Ok(classes) => Ok((StatusCode::OK, Json(classes))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get, path = "/api/classes/{id}",
    params(("id" = i64, Path, description = "Class ID")),
    responses((status = 200, description = "Class found", body = Class), (status = 404, description = "Not found")),
    tag = "Classes"
)]
pub async fn get_class_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match class_repo::get_class_by_id(&state.db_pool, id).await {
        Ok(Some(class_obj)) => Ok((StatusCode::OK, Json(json!(class_obj)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Class not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[utoipa::path(
    delete, path = "/api/classes/{id}",
    params(("id" = i64, Path, description = "Class ID")),
    responses((status = 200, description = "Deleted successfully"), (status = 404, description = "Not found")),
    tag = "Classes"
)]
pub async fn delete_class(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match class_repo::delete_class(&state.db_pool, id).await {
        Ok(0) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Class not found" })),
        )),
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({ "message": "Class deleted successfully" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
