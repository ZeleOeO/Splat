use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use sea_orm::DbErr;

pub enum AppError {
    DbError(DbErr),
    Unauthorized,
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DbError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
        }
    }
}
