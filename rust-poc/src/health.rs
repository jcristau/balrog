use axum::{
    extract::State,
    http::StatusCode,
};
use sqlx::MySqlPool;

pub async fn lbheartbeat() -> StatusCode {
    StatusCode::OK
}

pub async fn heartbeat(State(pool): State<MySqlPool>) -> StatusCode {
    match sqlx::query("SELECT 1 FROM rules LIMIT 1")
        .fetch_optional(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
