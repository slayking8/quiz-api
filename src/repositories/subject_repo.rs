use crate::models::subject::{CreateSubjectPayload, Subject};
use sqlx::SqlitePool;

// ==========================================
// REPOSITORY: SUBJECT
// ==========================================

pub async fn create_subject(
    pool: &SqlitePool,
    payload: &CreateSubjectPayload,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO subjects (name) VALUES (?)")
        .bind(&payload.name)
        .execute(pool)
        .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_all_subjects(pool: &SqlitePool) -> Result<Vec<Subject>, sqlx::Error> {
    let subjects = sqlx::query_as::<_, Subject>("SELECT id, name, created_at FROM subjects")
        .fetch_all(pool)
        .await?;

    Ok(subjects)
}

pub async fn get_subject_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Subject>, sqlx::Error> {
    sqlx::query_as::<_, Subject>("SELECT id, name, created_at FROM subjects WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_subject(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM subjects WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
