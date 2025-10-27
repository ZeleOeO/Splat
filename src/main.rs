use axum::{extract::{ State}, response::{IntoResponse}, routing::{get, post}, Router};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

use crate::{db::db::init_db, middleware::auth_middleware::AuthUser, services::{auth_service::{login_user, register_user}, bills_service::create_bill}};

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
    .route("/register", post(register_user))
    .route("/login", post(login_user))
    .route("/profile", get(welcome_user))
    .route("/bills/new", post(create_bill))
    .with_state(db);

    let addr = "0.0.0.0:6000";
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[axum::debug_handler]
async fn welcome_user(auth: AuthUser) -> impl IntoResponse {
    format!("Hello, user {}", auth.0)
}
