use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, HeaderMap},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{errors::error::AppError, services::auth_service::Claims};

#[derive(Clone)]
pub struct AuthUser(pub String);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state).await.map_err(|_| AppError::Unauthorized)?;
        let bearer = headers.get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;
        let token_data = decode::<Claims>(bearer, &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()), &Validation::default()).map_err(|_| AppError::Unauthorized)?;
        Ok(AuthUser(token_data.claims.sub))
    }
}
