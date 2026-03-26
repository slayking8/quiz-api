use crate::models::score::{Score, ScoreSyncPayload};
use sqlx::SqlitePool;

// ==========================================
// REPOSITORY: SCORE
// ==========================================

/// Inserts multiple scores at once.
/// Uses INSERT OR IGNORE to guarantee idempotency. If the frontend sends the same
/// local_attempt_id twice (due to network retries), the database will silently
/// ignore the duplicate without failing the transaction.
pub async fn sync_scores(
    pool: &SqlitePool,
    payloads: &[ScoreSyncPayload],
) -> Result<u64, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let mut actually_inserted = 0;

    for payload in payloads {
        let result = sqlx::query(
            "INSERT OR IGNORE INTO scores (session_id, student_id, score, played_at, local_attempt_id) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(payload.session_id)
        .bind(payload.student_id)
        .bind(payload.score)
        .bind(&payload.played_at)
        .bind(&payload.local_attempt_id)
        .execute(&mut *tx)
        .await?;

        // rows_affected() will be 1 if it's a new UUID, or 0 if it was ignored
        actually_inserted += result.rows_affected();
    }

    tx.commit().await?;

    Ok(actually_inserted)
}

pub async fn get_scores_by_session(
    pool: &SqlitePool,
    session_id: i64,
) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>(
        "SELECT id, session_id, student_id, score, played_at, synced_at, local_attempt_id 
         FROM scores 
         WHERE session_id = ? 
         ORDER BY score DESC, played_at ASC",
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    Ok(scores)
}
