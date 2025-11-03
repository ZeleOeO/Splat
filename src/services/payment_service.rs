use axum::{extract::State, response::IntoResponse};
use sea_orm::DatabaseConnection;

use crate::errors::error::AppError;

// pub async fn handle_payment(
//     State(db): State<DatabaseConnection>
// ) -> Result<impl IntoResponse, AppError> {

// }
