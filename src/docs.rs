use utoipa::OpenApi;

use crate::handlers::{class_handler, score_handler, session_handler, student_handler, subject_handler, teacher_handler};
use crate::models::{class::*, score::*, session::*, student::*, subject::*, teacher::*};

// ==========================================
// OPENAPI (SWAGGER) CONFIGURATION
// ==========================================

#[derive(OpenApi)]
#[openapi(
    paths(
        teacher_handler::create_teacher,
        teacher_handler::get_teachers,
        teacher_handler::get_teacher_by_id,
        teacher_handler::delete_teacher,
        
        class_handler::create_class,
        class_handler::get_classes,
        class_handler::get_class_by_id,
        class_handler::delete_class,
        
        subject_handler::create_subject,
        subject_handler::get_subjects,
        subject_handler::get_subject_by_id,
        subject_handler::delete_subject,
        
        student_handler::create_student,
        student_handler::get_students,
        student_handler::get_student_by_id,
        student_handler::delete_student,
        student_handler::get_students_by_class,
        
        session_handler::create_session,
        session_handler::get_sessions,
        session_handler::get_session_by_id,
        session_handler::delete_session,
        session_handler::add_question_to_session,
        
        score_handler::sync_scores,
        score_handler::get_session_scores,
    ),
    components(
        schemas(
            Teacher, CreateTeacherPayload,
            Class, CreateClassPayload,
            Subject, CreateSubjectPayload,
            Student, CreateStudentPayload,
            Session, CreateSessionPayload, CreateQuestionPayload, CreateOptionPayload,
            Score, ScoreSyncPayload 
        )
    ),
    tags(
        (name = "Teachers", description = "Teacher management endpoints"),
        (name = "Classes", description = "Class management endpoints"),
        (name = "Subjects", description = "Subject management endpoints"),
        (name = "Students", description = "Student management endpoints"),
        (name = "Sessions", description = "Quiz Sessions management endpoints"),
        (name = "Scores", description = "Offline Synchronization and Leaderboard endpoints"),
    ),
    info(
        title = "Offline Quiz API",
        version = "1.0.0",
        description = "API documentation for the offline classroom quiz application."
    )
)]
pub struct ApiDoc;
