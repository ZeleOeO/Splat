use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, ModelTrait};
use futures::join;
use crate::{
    dto::{dto::ApiResponse, enums::BilleeStatus, requests::{BilleeCreateRequest, TransactionInititializationRequest}},
    entities::{billee::{ActiveModel as Billee, Entity}, wallet::Entity as Wallet},
    errors::error::AppError,
    middleware::{auth_middleware::AuthUser, http_client::fetch_process_paystack_payment},
    utils::{
        bills_utils::find_bill_by_id, mapper::billee_to_billeedto, user_utils::{find_user_by_unique_id, find_user_by_user_id},
    },
};


pub async fn add_billee_to_bill(
    auth: AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<BilleeCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    let userid = payload.user_id.unwrap_or_default();
    let mut user_id: Option<i32> = None;
    let user_future = async {
        if userid.is_empty() {
            None
        } else {
            Some(find_user_by_unique_id(&userid, &db).await)
        }
    };

    let (option_bill, auth_user_result, potential_user) = join!(
        find_bill_by_id(&payload.bill_id, &db),
        find_user_by_user_id(auth.0, &db),
        user_future
    );

    let auth_user = auth_user_result;

    if let Some(user_model) = potential_user {
        user_id = Some(user_model.id);
    }

    let bill = match option_bill {
        Some(b) => b,
        None => {
            return Ok(ApiResponse::api_response(
                StatusCode::NOT_FOUND.as_u16(),
                "Bill Not Found",
                None,
            ));
        }
    };

    let amount_user_due = (payload.percentage as f32 * bill.total_amount * 0.01).round();

    let total_percentage: i32 = bill.find_related(Entity).all(&db).await
        .unwrap_or_default()
        .iter()
        .map(|billee| billee.percentage)
        .sum();

    if payload.percentage + total_percentage > 100 {
        return Ok(ApiResponse::api_response(
            StatusCode::EXPECTATION_FAILED.as_u16(),
            "Percentage exceeds the maximum limit for this bill",
            None
        ));
    }

    let wallet_future = auth_user.find_related(Wallet).one(&db);

    let wallet = wallet_future.await.unwrap();
    let wallet = match wallet {
        Some(w) => w,
        None => {
            return Ok(ApiResponse::api_response(
                StatusCode::BAD_GATEWAY.as_u16(),
                "User doesn't have a wallet",
                None
            ));
        }
    };

    let transaction_init_request = TransactionInititializationRequest {
        email: payload.billee_email,
        amount: amount_user_due.to_string(),
        bearer: "subaccount".to_string(),
        subaccount: wallet.subaccount_code,
        transaction_charge: 20,
    };

    let paystack_payment = fetch_process_paystack_payment(&transaction_init_request)
        .await?;

    let payment = match paystack_payment {
        Some(payment) => payment,
        None => return Ok(ApiResponse::api_response(StatusCode::BAD_REQUEST.as_u16(), "Error generating payment_link", None)),
    };

    let billee = Billee {
        name: Set(payload.name),
        percentage: Set(payload.percentage),
        status: Set(BilleeStatus::Pending.to_string()),
        user_id: Set(user_id),
        amount_paid: Set(0 as f64),
        amount_due: Set(amount_user_due as f64),
        bill_id: Set(bill.id),
        payment_link: Set(payment.authorization_url),
        ..Default::default()
    };

    let saved_billee = billee.insert(&db).await?;

    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Billee Added",
        Some(billee_to_billeedto(&saved_billee)),
    ))
}
