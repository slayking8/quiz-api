use crate::models::session::{
    CreateQuestionPayload, CreateSessionPayload, OptionResponse, QuestionDbRow, QuestionResponse,
    Session,
};
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

/// Adds multiple questions and their options to a session in a SINGLE atomic transaction.
/// If any insert fails, the entire transaction is rolled back automatically.
pub async fn add_bulk_questions_to_session(
    pool: &SqlitePool,
    session_id: i64,
    questions: &[CreateQuestionPayload],
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut questions_inserted = 0;

    for question in questions {
        // 1. Insert Question
        let question_result = sqlx::query("INSERT INTO questions (session_id, text) VALUES (?, ?)")
            .bind(session_id)
            .bind(&question.text)
            .execute(&mut *tx)
            .await?;

        let question_id = question_result.last_insert_rowid();

        // 2. Insert Options
        for option in &question.options {
            sqlx::query("INSERT INTO options (question_id, text, is_correct) VALUES (?, ?, ?)")
                .bind(question_id)
                .bind(&option.text)
                .bind(option.is_correct)
                .execute(&mut *tx)
                .await?;
        }

        questions_inserted += 1;
    }

    tx.commit().await?;

    Ok(questions_inserted)
}

/// Fetches all questions for a specific session, including their options.
pub async fn get_questions_for_session(
    pool: &SqlitePool,
    session_id: i64,
) -> Result<Vec<QuestionResponse>, sqlx::Error> {
    // 1. Fetch all questions for the session
    let db_questions =
        sqlx::query_as::<_, QuestionDbRow>("SELECT id, text FROM questions WHERE session_id = ?")
            .bind(session_id)
            .fetch_all(pool)
            .await?;

    let mut full_questions = Vec::new();

    // 2. Fetch options for each question
    for q in db_questions {
        let options = sqlx::query_as::<_, OptionResponse>(
            "SELECT id, text, is_correct FROM options WHERE question_id = ?",
        )
        .bind(q.id)
        .fetch_all(pool)
        .await?;

        // Assemble the final nested structure
        full_questions.push(QuestionResponse {
            id: q.id,
            text: q.text,
            options,
        });
    }

    Ok(full_questions)
}

/// Updates the status of an existing session.
pub async fn update_session_status(
    pool: &SqlitePool,
    session_id: i64,
    status: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("UPDATE sessions SET status = ? WHERE id = ?")
        .bind(status)
        .bind(session_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
