use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use sea_orm::DbErr;

use crate::dto::dto::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    RequestError(reqwest::Error),
    DbError(DbErr),
    Unauthorized,
    ExpectationFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DbError(err) => ApiResponse::api_response(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), &err.to_string(), None::<String>).into_response(),
            AppError::Unauthorized => ApiResponse::api_response(StatusCode::UNAUTHORIZED.as_u16(), "Unauthorized", None::<String>).into_response(),
            AppError::ExpectationFailed(err) => ApiResponse::api_response(StatusCode::BAD_REQUEST.as_u16(), &err.to_string(), None::<String>).into_response(),
            AppError::RequestError(err) => ApiResponse::api_response(StatusCode::BAD_REQUEST.as_u16(), &err.to_string(), None::<String>).into_response()
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DbError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::RequestError(err)
    }
}

