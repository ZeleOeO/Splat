use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{dto::dto::{BillCreateRequest}, entities::bills::ActiveModel as Bill, errors::error::AppError, middleware::auth_middleware::AuthUser};

#[debug_handler]
pub async fn create_bill(
    auth: AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BillCreateRequest>
) -> Result<impl IntoResponse, AppError> {
    let bill = Bill {
        title: Set(payload.title),
        description: Set(payload.description),
        creator_id: Set(auth.0),
        total_amount: Set(payload.total_amount),
        status: Set("Open".to_string()),
        category: Set(payload.category),
        created_at: Set(Utc::now().naive_local()),
        due_date: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    let saved_bill = bill.insert(&db).await?;

    Ok((StatusCode::OK, "Bill Created"))

}

