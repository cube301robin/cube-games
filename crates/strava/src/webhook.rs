use serde::{Deserialize, Serialize};

/// Strava webhook event payload.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StravaEvent {
    pub object_type: String,   // "activity" | "athlete"
    pub object_id: u64,
    pub aspect_type: String,   // "create" | "update" | "delete"
    pub owner_id: i64,         // Strava athlete_id
    pub subscription_id: i32,
    pub event_time: i64,
    #[serde(default)]
    pub updates: serde_json::Value,
}

/// Query params for Strava's webhook subscription validation challenge.
#[derive(Debug, Deserialize)]
pub struct WebhookChallenge {
    #[serde(rename = "hub.mode")]
    pub mode: String,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: String,
    #[serde(rename = "hub.challenge")]
    pub challenge: String,
}
