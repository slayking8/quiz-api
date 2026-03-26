use crate::models::teacher::{CreateTeacherPayload, Teacher};
use sqlx::SqlitePool;

// ==========================================
// REPOSITORY: TEACHER
// ==========================================

/// Inserts a new teacher into the database and returns the generated ID.
pub async fn create_teacher(
    pool: &SqlitePool,
    payload: &CreateTeacherPayload,
) -> Result<i64, sqlx::Error> {
    // We use parameterized queries (?) to prevent SQL Injection attacks
    let result = sqlx::query("INSERT INTO teachers (name, email) VALUES (?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .execute(pool)
        .await?;

    // Return the ID of the newly created row
    Ok(result.last_insert_rowid())
}

/// Fetches all teachers from the database.
pub async fn get_all_teachers(pool: &SqlitePool) -> Result<Vec<Teacher>, sqlx::Error> {
    // query_as automatically maps the SQL result to our Teacher struct
    let teachers = sqlx::query_as::<_, Teacher>("SELECT id, name, email, created_at FROM teachers")
        .fetch_all(pool)
        .await?;

    Ok(teachers)
}

pub async fn get_teacher_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Teacher>, sqlx::Error> {
    sqlx::query_as::<_, Teacher>("SELECT id, name, email, created_at FROM teachers WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_teacher(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM teachers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
