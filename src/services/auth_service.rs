use axum::debug_handler;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::dto::dto::*;
use crate::entities::user::{self, ActiveModel as User};
use crate::entities::user::Entity as UserDB;
use crate::errors::error::AppError;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub iss: usize,
    pub exp: usize,
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

pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

#[debug_handler]
pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<UserRegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    if payload.password != payload.repeat_password {
        return Err(AppError::ExpectationFailed("Passwords don't match".into()));
    }

    let password_hash = hash_password(&payload.password);

    let user =  User {
        user_name: Set(payload.username),
        hashed_password: Set(password_hash),
        created_at: Set(chrono::Utc::now().naive_local()),
        first_name: Set(payload.first_name),
        last_name: Set(payload.last_name),
        email: Set(payload.email),
        ..Default::default()
    };

    user.insert(&db).await?;

    Ok((StatusCode::CREATED, "User Created"))
}

#[debug_handler]
pub async fn login_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<UserLoginRequest>
) -> impl IntoResponse {
    let user = UserDB::find()
    .filter(user::Column::UserName.eq(payload.username))
    .one(&db)
    .await.unwrap();

    if let Some(user) = user {
        if verify_password(&payload.password, &user.hashed_password) {
            let token = create_jwt(user.id);
            return (StatusCode::OK, token)
        }
    }

    (StatusCode::UNAUTHORIZED, "Invalid Credentials".to_string())
}
