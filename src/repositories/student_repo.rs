use crate::models::student::{CreateStudentPayload, Student};
use sqlx::SqlitePool;

// ==========================================
// REPOSITORY: STUDENT
// ==========================================

pub async fn create_student(
    pool: &SqlitePool,
    payload: &CreateStudentPayload,
) -> Result<i64, sqlx::Error> {
    let result =
        sqlx::query("INSERT INTO students (student_code, name, class_id) VALUES (?, ?, ?)")
            .bind(&payload.student_code)
            .bind(&payload.name)
            .bind(payload.class_id)
            .execute(pool)
            .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_all_students(pool: &SqlitePool) -> Result<Vec<Student>, sqlx::Error> {
    let students = sqlx::query_as::<_, Student>(
        "SELECT id, student_code, name, class_id, created_at FROM students",
    )
    .fetch_all(pool)
    .await?;

    Ok(students)
}

/// Fetches all students belonging to a specific class.
pub async fn get_students_by_class(
    pool: &SqlitePool,
    class_id: i64,
) -> Result<Vec<Student>, sqlx::Error> {
    let students = sqlx::query_as::<_, Student>(
        "SELECT id, student_code, name, class_id, created_at FROM students WHERE class_id = ?",
    )
    .bind(class_id)
    .fetch_all(pool)
    .await?;

    Ok(students)
}

pub async fn get_student_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Student>, sqlx::Error> {
    sqlx::query_as::<_, Student>(
        "SELECT id, student_code, name, class_id, created_at FROM students WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_student(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
