use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, ModelTrait};

use crate::{
    dto::{dto::ApiResponse, enums::BilleeStatus, requests::BilleeCreateRequest},
    entities::billee::{ActiveModel as Billee, Entity},
    errors::error::AppError,
    middleware::auth_middleware::AuthUser,
    utils::{
        bills_utils::find_bill_by_id, mapper::billee_to_billeedto, user_utils::find_user_by_unique_id,
    },
};

pub async fn add_billee_to_bill(
    _: AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BilleeCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    let userid = payload.user_id.unwrap_or("".to_string());
    let mut user_id: Option<i32> = None;
    if userid != "" {
        user_id = Some(find_user_by_unique_id(&userid, &db).await.id);
    }

    let option_bill = find_bill_by_id(&payload.bill_id, &db).await;
    if option_bill.is_none() {
        return Ok(ApiResponse::api_response(
            StatusCode::OK.as_u16(),
            "Bill Created",
            None,
        ));
    }
    let bill = option_bill.unwrap();
    let amount_user_due = ((payload.percentage * bill.total_amount) as f64 * 0.01) as f64;

    let total_percentage: i32 = bill.find_related(Entity).all(&db).await.unwrap().iter().map(|billee| {
        billee.percentage
    }).sum();

    if payload.percentage + total_percentage > 100 {
        return Ok(ApiResponse::api_response(StatusCode::EXPECTATION_FAILED.as_u16(), "You've exceeded the number of billees on this bill", None));
    }

    let billee = Billee {
        name: Set(payload.name),
        percentage: Set(payload.percentage),
        status: Set(BilleeStatus::Pending.to_string()),
        user_id: Set(user_id),
        amount_paid: Set(0 as f64),
        amount_due: Set(amount_user_due as f64),
        bill_id: Set(bill.id),
        payment_details: Set(serde_json::Value::Null),
        ..Default::default()
    };

    let saved_billee = billee.insert(&db).await?;
    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Billee Added",
        Some(billee_to_billeedto(&saved_billee)),
    ))
}
