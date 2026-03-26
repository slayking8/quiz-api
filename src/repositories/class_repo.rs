use crate::models::class::{Class, CreateClassPayload};
use sqlx::SqlitePool;

// ==========================================
// REPOSITORY: CLASS
// ==========================================

pub async fn create_class(
    pool: &SqlitePool,
    payload: &CreateClassPayload,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO classes (name) VALUES (?)")
        .bind(&payload.name)
        .execute(pool)
        .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_all_classes(pool: &SqlitePool) -> Result<Vec<Class>, sqlx::Error> {
    let classes = sqlx::query_as::<_, Class>("SELECT id, name, created_at FROM classes")
        .fetch_all(pool)
        .await?;

    Ok(classes)
}

pub async fn get_class_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Class>, sqlx::Error> {
    sqlx::query_as::<_, Class>("SELECT id, name, created_at FROM classes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_class(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM classes WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
