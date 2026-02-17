use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(String),
    InvalidInput(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Error: {:?}", self);

        // Return empty updates XML on all errors
        let xml = "<?xml version=\"1.0\"?>\n<updates></updates>";

        (
            StatusCode::OK,
            [("Content-Type", "text/xml")],
            xml,
        ).into_response()
    }
}
