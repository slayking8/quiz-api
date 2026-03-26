use crate::models::session::{CreateQuestionPayload, CreateSessionPayload, Session};
use sqlx::SqlitePool;

// ==========================================
// REPOSITORY: SESSION
// ==========================================

/// Creates a new, empty Session.
pub async fn create_session(
    pool: &SqlitePool,
    payload: &CreateSessionPayload,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO sessions (title, teacher_id, class_id, subject_id, status, time_limit_seconds) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&payload.title)
    .bind(payload.teacher_id)
    .bind(payload.class_id)
    .bind(payload.subject_id)
    .bind(&payload.status)
    .bind(payload.time_limit_seconds)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Adds a single Question and its Options to an existing Session.
/// Uses a transaction to ensure options are always saved alongside the question.
pub async fn add_question_to_session(
    pool: &SqlitePool,
    session_id: i64,
    payload: &CreateQuestionPayload,
) -> Result<i64, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // 1. Insert the Question
    let question_result = sqlx::query("INSERT INTO questions (session_id, text) VALUES (?, ?)")
        .bind(session_id)
        .bind(&payload.text)
        .execute(&mut *tx)
        .await?;

    let question_id = question_result.last_insert_rowid();

    // 2. Insert the Options for this Question
    for option in &payload.options {
        sqlx::query("INSERT INTO options (question_id, text, is_correct) VALUES (?, ?, ?)")
            .bind(question_id)
            .bind(&option.text)
            .bind(option.is_correct)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(question_id)
}

/// Fetches all sessions from the database.
pub async fn get_all_sessions(pool: &SqlitePool) -> Result<Vec<Session>, sqlx::Error> {
    let sessions = sqlx::query_as::<_, Session>(
        "SELECT id, title, teacher_id, class_id, subject_id, status, time_limit_seconds, created_at 
         FROM sessions"
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}

pub async fn get_session_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        "SELECT id, title, teacher_id, class_id, subject_id, status, time_limit_seconds, created_at FROM sessions WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_session(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    // Cascades to questions, options, and scores automatically!
    let result = sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
