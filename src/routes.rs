use axum::{
    Router,
    routing::{get, patch, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;
use crate::docs::ApiDoc;
use crate::handlers::{
    class_handler, dashboard_handler, score_handler, session_handler, student_handler,
    subject_handler, teacher_handler,
};

// ==========================================
// ROUTER SETUP
// ==========================================

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/admin", get(dashboard_handler::get_dashboard))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Teachers
        .route(
            "/api/teachers",
            post(teacher_handler::create_teacher).get(teacher_handler::get_teachers),
        )
        .route(
            "/api/teachers/:id",
            get(teacher_handler::get_teacher_by_id).delete(teacher_handler::delete_teacher),
        )
        // Subjects
        .route(
            "/api/subjects",
            post(subject_handler::create_subject).get(subject_handler::get_subjects),
        )
        .route(
            "/api/subjects/:id",
            get(subject_handler::get_subject_by_id).delete(subject_handler::delete_subject),
        )
        // Classes
        .route(
            "/api/classes",
            post(class_handler::create_class).get(class_handler::get_classes),
        )
        .route(
            "/api/classes/:id",
            get(class_handler::get_class_by_id).delete(class_handler::delete_class),
        )
        // Students
        .route(
            "/api/students",
            post(student_handler::create_student).get(student_handler::get_students),
        )
        .route(
            "/api/students/:id",
            get(student_handler::get_student_by_id).delete(student_handler::delete_student),
        )
        .route(
            "/api/classes/:class_id/students",
            get(student_handler::get_students_by_class),
        )
        // Sessions
        .route(
            "/api/sessions/:session_id/status",
            patch(session_handler::update_session_status),
        )
        .route(
            "/api/sessions",
            post(session_handler::create_session).get(session_handler::get_sessions),
        )
        .route(
            "/api/sessions/:id",
            get(session_handler::get_session_by_id).delete(session_handler::delete_session),
        )
        .route(
            "/api/sessions/:session_id/questions",
            post(session_handler::add_question_to_session)
                .get(session_handler::get_session_questions),
        )
        .route(
            "/api/sessions/:session_id/questions/bulk-text",
            post(session_handler::upload_bulk_questions_text),
        )
        // Scores
        .route("/api/scores/sync", post(score_handler::sync_scores))
        .route(
            "/api/sessions/:session_id/scores",
            get(score_handler::get_session_scores),
        )
        .with_state(state)
}
