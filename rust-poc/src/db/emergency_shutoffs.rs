use sqlx::MySqlPool;
use crate::error::AppError;

/// Checks if there's an emergency shutoff for the given product and channel
pub async fn is_emergency_shutoff(
    pool: &MySqlPool,
    product: &str,
    channel: &str,
) -> Result<bool, AppError> {
    let result = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM emergency_shutoffs WHERE product = ? AND channel = ? LIMIT 1"
    )
    .bind(product)
    .bind(channel)
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}
