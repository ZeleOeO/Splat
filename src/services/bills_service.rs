use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, ModelTrait};

use crate::{
    dto::{
        dto::{ApiResponse, BilleeDTO, BillsDTO},
        enums::BillsStatus,
        requests::BillCreateRequest,
    },
    entities::{
        billee::Entity as BilleeEntity, bills::ActiveModel as Bill,
        user_bills_joined::ActiveModel as BillsJoined,
    },
    errors::error::AppError,
    middleware::auth_middleware::AuthUser,
    utils::{
        bills_utils::{find_bill_by_id, find_bills_by_users},
        mapper::{bill_to_billdto, billee_to_billeedto},
    },
};

#[debug_handler]
pub async fn create_bill(
    auth: AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BillCreateRequest>,
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
        unique_id: Set(uuid::Uuid::new_v4().to_string()),
        ..Default::default()
    };

    let saved_bill = bill.insert(&db).await?;

    let bills_joined = BillsJoined {
        user_id: Set(auth.0),
        bills_joined_id: Set(saved_bill.id),
    };

    bills_joined.insert(&db).await?;

    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Bill Created",
        Some(bill_to_billdto(&saved_bill)),
    ))
}

pub async fn get_billeesfrom_bill(
    _: AuthUser,
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let bill = find_bill_by_id(&id, &db).await;

    if let Some(bill_stuff) = bill {
        let billees = bill_stuff.find_related(BilleeEntity).all(&db).await?;

        let billies_mapped: Vec<BilleeDTO> = billees
            .iter()
            .map(|billee| billee_to_billeedto(billee))
            .collect();

        return Ok(ApiResponse::api_response(
            StatusCode::OK.as_u16(),
            "Users Retrieved",
            Some(billies_mapped),
        ));
    }
    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Bill with ID not Found",
        None,
    ))
}

pub async fn get_bill_by_id(
    _: AuthUser,
    Path(id): Path<String>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let bill = find_bill_by_id(&id, &db).await;

    if let Some(bill_stuff) = bill {
        return Ok(ApiResponse::api_response(
            StatusCode::OK.as_u16(),
            "Bill Retrieved",
            Some(bill_to_billdto(&bill_stuff)),
        ));
    }
    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Bill with ID not Found",
        None,
    ))
}

pub async fn get_bills_by_user_id(
    auth: AuthUser,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let user_bills = find_bills_by_users(auth.0, &db).await?;

    let user_bills_dto: Vec<BillsDTO> = user_bills
        .iter()
        .map(|bill| bill_to_billdto(bill))
        .collect();

    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Users Bills Retrieved",
        Some(user_bills_dto),
    ))
}
