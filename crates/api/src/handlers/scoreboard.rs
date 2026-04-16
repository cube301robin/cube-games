use axum::{extract::{Query, State}, Json};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, AppState};

fn current_iso_week() -> (i32, i32) {
    let now = Utc::now();
    (now.year(), now.iso_week().week() as i32)
}

#[derive(Deserialize)]
pub struct LeaderboardQuery {
    pub game: Option<String>,
    pub year: Option<i32>,
    pub week: Option<i32>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Serialize)]
pub struct IndividualLeaderboardResponse {
    pub game: String,
    pub year: i32,
    pub week: i32,
    pub page: i64,
    pub per_page: i64,
    pub entries: Vec<db::models::IndividualLeaderboardEntry>,
}

pub async fn individual_leaderboard(
    State(state): State<AppState>,
    Query(q): Query<LeaderboardQuery>,
) -> Result<Json<IndividualLeaderboardResponse>, ApiError> {
    let (cur_year, cur_week) = current_iso_week();
    let game = q.game.unwrap_or_else(|| "walk_run_2026".into());
    let year = q.year.unwrap_or(cur_year);
    let week = q.week.unwrap_or(cur_week);
    let page = q.page.unwrap_or(1).max(1);
    let per_page = q.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let entries = db::queries::get_individual_leaderboard(
        &state.db, &game, year, week, per_page, offset,
    )
    .await?;

    Ok(Json(IndividualLeaderboardResponse {
        game,
        year,
        week,
        page,
        per_page,
        entries,
    }))
}

#[derive(Serialize)]
pub struct TeamLeaderboardResponse {
    pub game: String,
    pub year: i32,
    pub week: i32,
    pub entries: Vec<db::models::TeamLeaderboardEntry>,
}

pub async fn team_leaderboard(
    State(state): State<AppState>,
    Query(q): Query<LeaderboardQuery>,
) -> Result<Json<TeamLeaderboardResponse>, ApiError> {
    let (cur_year, cur_week) = current_iso_week();
    let game = q.game.unwrap_or_else(|| "walk_run_2026".into());
    let year = q.year.unwrap_or(cur_year);
    let week = q.week.unwrap_or(cur_week);

    let entries = db::queries::get_team_leaderboard(&state.db, &game, year, week).await?;

    Ok(Json(TeamLeaderboardResponse {
        game,
        year,
        week,
        entries,
    }))
}
