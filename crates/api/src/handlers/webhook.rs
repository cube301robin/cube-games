use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use strava::webhook::{StravaEvent, WebhookChallenge};

use crate::{error::ApiError, AppState};

/// GET /api/webhook/strava — subscription verification challenge
pub async fn strava_verify(
    State(state): State<AppState>,
    Query(q): Query<WebhookChallenge>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if q.verify_token != state.config.strava.webhook_verify_token {
        return Err((StatusCode::FORBIDDEN, "bad verify token".into()));
    }
    Ok(Json(json!({ "hub.challenge": q.challenge })))
}

/// POST /api/webhook/strava — receive activity events
pub async fn strava_event(
    State(state): State<AppState>,
    Json(event): Json<StravaEvent>,
) -> Result<StatusCode, ApiError> {
    // Only queue create events for activities
    if event.object_type != "activity" || event.aspect_type != "create" {
        return Ok(StatusCode::OK);
    }

    let event_id = format!("{}:{}", event.subscription_id, event.object_id);
    let payload = serde_json::to_value(&event).map_err(anyhow::Error::from)?;

    sqlx::query!(
        "INSERT INTO webhook_events (source, event_id, payload)
         VALUES ('strava', $1, $2)
         ON CONFLICT DO NOTHING",
        event_id,
        payload,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::OK)
}
