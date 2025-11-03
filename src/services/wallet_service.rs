use axum::{Json, extract::{State}, response::IntoResponse};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{dto::{dto::{ApiResponse, WalletDTO}, requests::WalletCreateRequest}, entities::wallet::ActiveModel as Wallet, errors::error::AppError, middleware::{auth_middleware::AuthUser, http_client::fetch_verify_wallet_creation}, utils::user_utils::find_user_by_user_id};

pub async fn create_wallet(
    auth_user: AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<WalletCreateRequest>
) -> Result<impl IntoResponse, AppError> {
    let user = find_user_by_user_id(auth_user.0, &db).await;
    let request = WalletCreateRequest {
        business_name: Some(user.user_name),
        percentage_charge: Some(2),
        ..payload
    };
    let sub_verified = fetch_verify_wallet_creation(&request).await?;
    if sub_verified.is_none() {
        return Ok(ApiResponse::api_response(StatusCode::BAD_GATEWAY.as_u16(), "Error Creating Wallet", None::<WalletDTO>));
    }
    let wallet_dto = sub_verified.unwrap();
    let save_wallet = wallet_dto.clone();
    let wallet = Wallet {
        user_id: Set(user.id),
        business_name: Set(wallet_dto.business_name),
        account_number: Set(wallet_dto.account_number),
        settlement_bank: Set(wallet_dto.settlement_bank),
        account_name: Set(wallet_dto.account_name),
        currency: Set(wallet_dto.currency),
        subaccount_code: Set(wallet_dto.subaccount_code),
        ..Default::default()
    };

    let _ = wallet.insert(&db).await?;

    Ok(ApiResponse::api_response(StatusCode::OK.as_u16(), "Wallet Created", Some(save_wallet.clone())))
}
