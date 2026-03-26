use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::AppState;
use crate::models::student::{CreateStudentPayload, Student};
use crate::repositories::student_repo;

#[utoipa::path(
    post,
    path = "/api/students",
    request_body = CreateStudentPayload,
    responses(
        (status = 201, description = "Student created successfully"),
        (status = 500, description = "Database error, duplicate code, or invalid class_id")
    ),
    tag = "Students"
)]
pub async fn create_student(
    State(state): State<AppState>,
    Json(payload): Json<CreateStudentPayload>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match student_repo::create_student(&state.db_pool, &payload).await {
        Ok(id) => {
            let response = json!({ "id": id, "message": "Student created successfully" });
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
    path = "/api/students",
    responses(
        (status = 200, description = "List of all students", body = [Student])
    ),
    tag = "Students"
)]
pub async fn get_students(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Student>>), (StatusCode, Json<Value>)> {
    match student_repo::get_all_students(&state.db_pool).await {
        Ok(students) => Ok((StatusCode::OK, Json(students))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/classes/{class_id}/students",
    params(
        ("class_id" = i64, Path, description = "The database ID of the class", example = 1)
    ),
    responses(
        (status = 200, description = "List of students in the specified class", body = [Student])
    ),
    tag = "Students"
)]
pub async fn get_students_by_class(
    State(state): State<AppState>,
    Path(class_id): Path<i64>,
) -> Result<(StatusCode, Json<Vec<Student>>), (StatusCode, Json<Value>)> {
    match student_repo::get_students_by_class(&state.db_pool, class_id).await {
        Ok(students) => Ok((StatusCode::OK, Json(students))),
        Err(e) => {
            let error_response = json!({ "error": e.to_string() });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

#[utoipa::path(
    get, path = "/api/students/{id}",
    params(("id" = i64, Path, description = "Student ID")),
    responses((status = 200, description = "Student found", body = Student), (status = 404, description = "Not found")),
    tag = "Students"
)]
pub async fn get_student_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match student_repo::get_student_by_id(&state.db_pool, id).await {
        Ok(Some(student)) => Ok((StatusCode::OK, Json(json!(student)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Student not found" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

#[utoipa::path(
    delete, path = "/api/students/{id}",
    params(("id" = i64, Path, description = "Student ID")),
    responses((status = 200, description = "Deleted successfully"), (status = 404, description = "Not found")),
    tag = "Students"
)]
pub async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match student_repo::delete_student(&state.db_pool, id).await {
        Ok(0) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Student not found" })),
        )),
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({ "message": "Student deleted successfully" })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}
