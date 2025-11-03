use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{dto::{dto::{BankDTO, TransactionInitializationDTO, WalletDTO}, requests::{TransactionInititializationRequest, WalletCreateRequest}},  errors::error::AppError};

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiResponse<T> {
    status: bool,
    message: String,
    data: T
}

pub async fn fetch_banks_from_paystack() -> Result<Vec<BankDTO>, AppError> {
    let sercret_key = std::env::var("PAYSTACK_SECRET_KEY").expect("PAYSTACK_SECRET_KEY not found");
    let res: ApiResponse<Vec<BankDTO>> = HTTP_CLIENT
    .get("https://api.paystack.co/bank")
    .bearer_auth(sercret_key)
    .send()
    .await
    .unwrap()
    .json()
    .await?;

    Ok(res.data)
}

pub async fn fetch_verify_wallet_creation(
    payload: &WalletCreateRequest
) -> Result<Option<WalletDTO>, AppError> {
    let sercret_key = std::env::var("PAYSTACK_SECRET_KEY").expect("PAYSTACK_SECRET_KEY not found");

    let res: ApiResponse<WalletDTO> = HTTP_CLIENT
    .post("https://api.paystack.co/subaccount")
    .bearer_auth(sercret_key)
    .json(payload)
    .send()
    .await?
    .json()
    .await?;

    if res.status == false {
        return Ok(None);
    }
    Ok(Some(res.data))
}

pub async fn fetch_process_paystack_payment(
    request: &TransactionInititializationRequest
) -> Result<Option<TransactionInitializationDTO>, AppError> {
    let sercret_key = std::env::var("PAYSTACK_SECRET_KEY").expect("PAYSTACK_SECRET_KEY not found");

    let res: ApiResponse<TransactionInitializationDTO> = HTTP_CLIENT
    .post("https://api.paystack.co/transaction/initialize")
    .bearer_auth(sercret_key)
    .json(request)
    .send()
    .await?
    .json()
    .await?;

    if res.status == false {
        return Ok(None);
    }
    Ok(Some(res.data))
}
