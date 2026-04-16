use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,      // user UUID
    pub user_id: i32,
    pub is_admin: bool,
    pub exp: i64,
}

pub fn create_token(
    user_id: i32,
    user_uuid: &str,
    is_admin: bool,
    secret: &str,
    duration_hours: i64,
) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::hours(duration_hours)).timestamp();
    let claims = Claims {
        sub: user_uuid.to_string(),
        user_id,
        is_admin,
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

/// Extractor for authenticated requests.
#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Unauthorized("missing bearer token".into()))?;

        let claims = verify_token(bearer.token(), &state.config.auth.jwt_secret)
            .map_err(|_| ApiError::Unauthorized("invalid or expired token".into()))?;

        Ok(AuthUser(claims))
    }
}

/// Extractor that also enforces admin role.
pub struct AdminUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AuthUser(claims) = AuthUser::from_request_parts(parts, state).await?;
        if !claims.is_admin {
            return Err(ApiError::Unauthorized("admin required".into()));
        }
        Ok(AdminUser(claims))
    }
}
