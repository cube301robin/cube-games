use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct Team {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub team_id: Option<i32>,
    pub strava_athlete_id: Option<i64>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub config: serde_json::Value,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct Activity {
    pub id: i64,
    pub external_id: String,
    pub source: String,
    pub user_id: i32,
    pub game_id: String,
    pub activity_type: String,
    pub value: f64,
    pub occurred_at: DateTime<Utc>,
    pub inserted_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct WeeklyIndividualScore {
    pub id: i64,
    pub user_id: i32,
    pub team_id: Option<i32>,
    pub game_id: String,
    pub year: i32,
    pub iso_week: i32,
    pub raw_value: f64,
    pub tier_name: Option<String>,
    pub base_points: f64,
    pub bonus_points: f64,
    pub total_points: f64,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct WeeklyTeamScore {
    pub team_id: i32,
    pub game_id: String,
    pub year: i32,
    pub iso_week: i32,
    pub avg_score: f64,
    pub active_member_count: i64,
    pub total_member_count: i64,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct ScoreAdjustment {
    pub id: i64,
    pub team_id: Option<i32>,
    pub user_id: Option<i32>,
    pub game_id: Option<String>,
    pub year: i32,
    pub month: Option<i32>,
    pub iso_week: Option<i32>,
    pub adjustment_type: String,
    pub points: f64,
    pub reason: String,
    pub created_by: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ---- Response DTOs ----

#[derive(Debug, Serialize)]
pub struct IndividualLeaderboardEntry {
    pub rank: i64,
    pub user_id: i32,
    pub user_name: String,
    pub team_name: Option<String>,
    pub tier_name: Option<String>,
    pub raw_value: f64,
    pub total_points: f64,
}

#[derive(Debug, Serialize)]
pub struct TeamLeaderboardEntry {
    pub rank: i64,
    pub team_id: i32,
    pub team_name: String,
    pub avg_score: f64,
    pub active_member_count: i64,
    pub total_member_count: i64,
}
