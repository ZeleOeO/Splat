use axum::debug_handler;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid;

use crate::dto::dto::*;
use crate::dto::requests::{RefreshTokenRequest, UserLoginRequest, UserRegisterRequest};
use crate::entities::user::Entity as UserDB;
use crate::entities::user::{self, ActiveModel as User};
use crate::errors::error::AppError;
use crate::middleware::auth_middleware::{AuthUser};
use crate::utils::mapper;
use crate::utils::user_utils::find_user_by_user_id;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub iss: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshClaims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub refresh_token: String,
}
fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

fn create_jwt(user_id: i32) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let issued_at = chrono::Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        iss: issued_at,
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

fn create_refresh_token(user_id: i32) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .unwrap()
        .timestamp() as usize;

    let claims = RefreshClaims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_REFRESH_SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

#[debug_handler]
pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<UserRegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Registering User");
    if payload.password != payload.repeat_password {
        error!("Passwords Don't Match");
        return Err(AppError::ExpectationFailed("Passwords don't match".into()));
    }

    let password_hash = hash_password(&payload.password);

    let user = User {
        user_name: Set(payload.username),
        hashed_password: Set(password_hash),
        created_at: Set(chrono::Utc::now().naive_local()),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        email: Set(payload.email),
        unique_id: Set(uuid::Uuid::new_v4().to_string()),
        ..Default::default()
    };

    let saved_user = match user.clone().insert(&db).await {
        Ok(u) => u,
        Err(e) => {
            error!("DB ERROR: {:?}", e);
            return Err(AppError::ExpectationFailed(format!("DB error: {:?}", e)));
        }
    };

    Ok(ApiResponse::api_response(
        400,
        "User Created",
        Some(mapper::user_to_userdto(&saved_user)),
    ))
}

#[debug_handler]
pub async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<UserLoginRequest>,
) -> impl IntoResponse {
    let user = UserDB::find()
        .filter(user::Column::UserName.eq(payload.username))
        .one(&db)
        .await
        .unwrap();

    if let Some(user) = user {
        if verify_password(&payload.password, &user.hashed_password) {
            let token = create_jwt(user.id);
            let refresh_token = create_refresh_token(user.id);
            return ApiResponse::api_response(
                StatusCode::OK.as_u16(),
                "User Logged in",
                Some(TokenResponse {
                    token: token,
                    refresh_token: refresh_token,
                }),
            );
        }
    }
    ApiResponse::api_response(
        StatusCode::UNAUTHORIZED.as_u16(),
        "Invalid Credentials",
        None::<TokenResponse>,
    )
}

#[debug_handler]
pub async fn refresh_token(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    use jsonwebtoken::{DecodingKey, Validation, decode};

    let token_data = decode::<RefreshClaims>(
        &payload.refresh_token,
        &DecodingKey::from_secret(std::env::var("JWT_REFRESH_SECRET").unwrap().as_ref()),
        &Validation::default(),
    )
    .unwrap();

    let user_id = token_data.claims.sub;

    let user = UserDB::find_by_id(user_id).one(&db).await.unwrap();
    if user.is_none() {
        return Ok(ApiResponse::api_response(
            StatusCode::UNAUTHORIZED.as_u16(),
            "Invalid Credentials",
            None::<TokenResponse>,
        ));
    }

    let new_access_token = create_jwt(user_id);

    let new_refresh_token = create_refresh_token(user_id);

    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "Token refreshed",
        Some(TokenResponse {
            token: new_access_token,
            refresh_token: new_refresh_token,
        }),
    ))
}

pub async fn get_profile_info (
    auth: AuthUser,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let user = find_user_by_user_id(auth.0, &db).await;

    Ok(ApiResponse::api_response(
        StatusCode::OK.as_u16(),
        "User Profile Retrieved",
        Some(mapper::user_to_userdto(&user)),
    ))
}
