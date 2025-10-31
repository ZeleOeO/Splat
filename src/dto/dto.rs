use axum::{Json, http::{StatusCode}, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub unique_id: String,
    pub user_name: String,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillsDTO {
    pub unique_id: String,
    pub title: String,
    pub description: Option<String>,
    pub total_amount: i32,
    pub status: String,
    pub category: String,
    pub created_at: String,
    pub due_date: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilleeDTO {
    pub name: String,
    pub percentage: i32,
    pub amount_due: f64,
    pub amount_paid: f64,
    pub status: String,
    pub user_id: Option<i32>,
    pub bill_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankDTO {
    pub name: String,
    pub code: String,
    pub country: String,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletDTO {
    pub business_name: String,
    pub account_number: String,
    pub settlement_bank: String,
    pub account_name: String,
    pub currency: String,
    pub subaccount_code: String
}


impl<T> ApiResponse<T> {
    pub fn api_response(status: u16, messsage: &str, data: Option<T>) -> Self {
        Self { status, message: messsage.to_string(), data:  data}

    }
}
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
