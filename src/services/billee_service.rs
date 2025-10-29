use axum::{Json, extract::{State}, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{dto::{dto::ApiResponse, enums::BilleeStatus, requests::BilleeCreateRequest}, entities::{billee::ActiveModel as Billee}, errors::error::AppError, middleware::auth_middleware::AuthUser, utils::{bills_utils::find_bill_by_id, mapper::billee_to_billeedto, user_utils::find_user_by_id}};

pub async fn add_billee_to_bill(
    _: AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BilleeCreateRequest>
) -> Result<impl IntoResponse, AppError> {
    let userid = payload.user_id.unwrap_or(0);
    let mut user_id: Option<i32> = None;
    if userid != 0 {
        user_id = Some(find_user_by_id(userid, &db).await.id);
    }

    let bill = find_bill_by_id(payload.bill_id, &db).await;
    let amount_user_due = ((payload.percentage * bill.total_amount) as f64 * 0.01) as f64;

    let billee = Billee {
        name: Set(payload.name),
        percentage: Set(payload.percentage),
        status: Set(BilleeStatus::Pending.to_string()),
        user_id: Set(user_id),
        amount_paid: Set(0 as f64),
        amount_due: Set(amount_user_due as f64),
        bill_id: Set(bill.id),
        ..Default::default()
    };

    let saved_billee = billee.insert(&db).await?;
    Ok(ApiResponse::api_response(StatusCode::OK.as_u16(), "Billee Added", Some(billee_to_billeedto(&saved_billee))))
}
