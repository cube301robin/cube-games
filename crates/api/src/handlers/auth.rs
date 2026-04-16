use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::{auth::create_token, error::ApiError, AppState};

pub async fn strava_login(State(state): State<AppState>) -> impl IntoResponse {
    let url = state.strava_oauth.authorization_url();
    Redirect::temporary(&url)
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    #[allow(dead_code)]
    pub scope: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
    pub name: String,
    pub is_admin: bool,
}

pub async fn strava_callback(
    State(state): State<AppState>,
    Query(q): Query<CallbackQuery>,
) -> Result<Json<AuthResponse>, ApiError> {
    let token_resp = state
        .strava_oauth
        .exchange_code(&q.code)
        .await
        .map_err(anyhow::Error::from)?;

    let expires_at = DateTime::from_timestamp(token_resp.expires_at, 0)
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("invalid expires_at")))?;

    let user = db::queries::upsert_strava_user(
        &state.db,
        token_resp.athlete.id,
        &token_resp.athlete.full_name(),
        token_resp.athlete.email.as_deref(),
        &token_resp.access_token,
        &token_resp.refresh_token,
        expires_at,
    )
    .await?;

    let jwt = create_token(
        user.id,
        &user.uuid.to_string(),
        user.is_admin,
        &state.config.auth.jwt_secret,
        state.config.auth.session_duration_hours,
    )
    .map_err(anyhow::Error::from)?;

    Ok(Json(AuthResponse {
        token: jwt,
        user_id: user.id,
        name: user.name,
        is_admin: user.is_admin,
    }))
}
