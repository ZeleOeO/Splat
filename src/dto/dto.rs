use axum::{Json, http::{StatusCode}, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    status: u16,
    message: String,
    data: Option<T>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub user_name: String,
    pub hashed_password: String,
    pub email: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillsDTO {
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
