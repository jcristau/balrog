use sqlx::MySqlPool;
use crate::db::models::PinnableReleaseRow;
use crate::error::AppError;

/// Looks up pinnable release mapping
pub async fn get_pinnable_release(
    pool: &MySqlPool,
    product: &str,
    channel: &str,
    version: &str,
) -> Result<Option<String>, AppError> {
    let row: Option<PinnableReleaseRow> = sqlx::query_as(
        "SELECT product, channel, version, mapping FROM pinnable_releases WHERE product = ? AND channel = ? AND version = ?"
    )
    .bind(product)
    .bind(channel)
    .bind(version)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.mapping))
}
