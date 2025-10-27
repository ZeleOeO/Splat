use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use sea_orm::DbErr;

use crate::dto::dto::ApiResponse;

pub enum AppError {
    DbError(DbErr),
    Unauthorized,
    NotFound,
    ExpectationFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DbError(err) => ApiResponse::api_response(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), &err.to_string(), None::<String>).into_response(),
            AppError::Unauthorized => ApiResponse::api_response(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized", None::<String>).into_response(),
            AppError::NotFound => ApiResponse::api_response(StatusCode::NOT_FOUND.as_u16(), "Url Not Found", None::<String>).into_response(),
            AppError::ExpectationFailed(err) => ApiResponse::api_response(StatusCode::BAD_REQUEST.as_u16(), &err.to_string(), None::<String>).into_response()
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DbError(err)
    }
}

