use axum::{Json, debug_handler, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{dto::{dto::{ApiResponse, BillCreateRequest}, enums::BillsStatus}, entities::{bills::{ActiveModel as Bill, Entity as BillDB}, prelude::UserBillsJoined, user_bills_joined::ActiveModel as BillsJoined}, errors::error::AppError, middleware::auth_middleware::AuthUser, utils::mapper::bill_to_billdto};

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
        status: Set(BillsStatus::Open.to_string()),
        category: Set(payload.category),
        created_at: Set(Utc::now().naive_local()),
        due_date: Set(Utc::now().naive_local() + Duration::days(payload.days_till_due)),
        ..Default::default()
    };

    let saved_bill = bill.insert(&db).await?;

    let bills_joined = BillsJoined {
        user_id: Set(auth.0),
        bills_joined_id: Set(saved_bill.id)
    };

    bills_joined.insert(&db).await?;

    Ok(ApiResponse::api_response(StatusCode::OK.as_u16(), "Bill Created", Some(bill_to_billdto(&saved_bill))))
}

pub async fn join_bill(
    auth: AuthUser,
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>
) -> Result<impl IntoResponse, AppError> {
    let bills = BillDB::find_by_id(id).one(&db).await.unwrap();

    if let Some(bill) = bills {
        let _ = BillsJoined {
            user_id: Set(auth.0),
            bills_joined_id: Set(bill.id)
        }.insert(&db).await?;

        return Ok(ApiResponse::api_response(StatusCode::OK.as_u16(), "User joined Bill", Some(bill_to_billdto(&bill))));
    }

    Ok(ApiResponse::api_response(StatusCode::EXPECTATION_FAILED.as_u16(), "Failed to add user to bill", None))
}
