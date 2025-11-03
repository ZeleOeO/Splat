use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use tokio::{task::JoinSet};

use crate::{
    dto::dto::ApiResponse, entities::bank::{ActiveModel as BankModel}, errors::error::AppError,
    middleware::http_client::fetch_banks_from_paystack,
};

pub async fn populate_bank_table(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let banks = fetch_banks_from_paystack().await?;
    let mut set = JoinSet::new();

    for bank in banks {
        let value = db.clone();
        set.spawn(async move {
            let created_bank = BankModel {
                name: Set(bank.name),
                code: Set(bank.code),
                country: Set(bank.country),
                currency: Set(bank.currency),
                ..Default::default()
            };
            let _ = created_bank.insert(&value).await.unwrap();
        });
    };
    while let Some(res) = set.join_next().await {
        if let Err(e) = res {
            eprintln!("Insert task failed: {:?}", e);
        }
    }

    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "DB populated",
        None::<String>,
    ))
}
