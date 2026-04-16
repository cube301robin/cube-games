use axum::{extract::State, http::StatusCode, Json};
use chrono::{Datelike, Utc};
use serde::Deserialize;

use crate::{auth::AdminUser, error::ApiError, AppState};

#[derive(Deserialize)]
pub struct RecomputeRequest {
    pub game_id: Option<String>,
    pub year: Option<i32>,
    pub week: Option<u32>,
}

pub async fn recompute(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(req): Json<RecomputeRequest>,
) -> Result<StatusCode, ApiError> {
    let now = Utc::now();
    let year = req.year.unwrap_or(now.year());
    let week = req.week.unwrap_or(now.iso_week().week());

    let games: Vec<String> = if let Some(id) = req.game_id {
        vec![id]
    } else {
        sqlx::query_scalar!("SELECT id FROM games WHERE active = true")
            .fetch_all(&state.db)
            .await?
    };

    for game_id in games {
        if let Some(scorer) = state.scoring.get(&game_id) {
            let ctx =
                scoring::ScoringContext::for_week(state.db.clone(), &game_id, year, week);
            scorer
                .compute_weekly_individual(&ctx)
                .await
                .map_err(anyhow::Error::from)?;
            scorer
                .compute_weekly_team(&ctx)
                .await
                .map_err(anyhow::Error::from)?;
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct AdjustmentRequest {
    pub team_id: i32,
    pub game_id: Option<String>,
    pub year: i32,
    pub month: i32,
    pub points: f64,
    pub reason: String,
}

pub async fn add_adjustment(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(req): Json<AdjustmentRequest>,
) -> Result<StatusCode, ApiError> {
    let game_id = req.game_id.as_deref().unwrap_or("global");

    sqlx::query!(
        "INSERT INTO score_adjustments
           (team_id, game_id, year, month, adjustment_type, points, reason, created_by)
         VALUES ($1,$2,$3,$4,'special_card',$5,$6,$7)",
        req.team_id,
        game_id,
        req.year,
        req.month,
        req.points,
        req.reason,
        admin.0.user_id,
    )
    .execute(&state.db)
    .await?;

    sqlx::query!(
        "INSERT INTO monthly_team_state
           (team_id, game_id, year, month, special_card_modifier)
         VALUES ($1,$2,$3,$4,$5)
         ON CONFLICT (team_id, game_id, year, month) DO UPDATE SET
           special_card_modifier =
             monthly_team_state.special_card_modifier + EXCLUDED.special_card_modifier,
           updated_at = now()",
        req.team_id,
        game_id,
        req.year,
        req.month,
        req.points,
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::CREATED)
}
