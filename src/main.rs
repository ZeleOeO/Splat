use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;

use crate::{db::db::init_db, services::{auth_service::{login_user, register_user}, bank_service::populate_bank_table, billee_service::add_billee_to_bill, bills_service::{create_bill, get_bill_by_id, get_billeesfrom_bill, get_bills_by_user_id}, wallet_service::create_wallet}};

mod db;
mod services;
mod dto;
mod entities;
mod errors;
mod middleware;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let db = init_db().await;

    let app = Router::new()
    .route("/auth/register", post(register_user))
    .route("/auth/login", post(login_user))
    .route("/bills/new", post(create_bill))
    .route("/bills/add-billee", post(add_billee_to_bill))
    .route("/bills/billees/{id}", get(get_billeesfrom_bill))
    .route("/bills/{id}", get(get_bill_by_id))
    .route("/bills/users", get(get_bills_by_user_id))
    .route("/admin/populate-bank", get(populate_bank_table))
    .route("/wallet/new", post(create_wallet))
    .with_state(db);

    let addr = "0.0.0.0:6000";
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

