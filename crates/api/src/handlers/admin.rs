use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{Datelike, Utc};
use rust_xlsxwriter::{Color, Format, Workbook};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

#[derive(Deserialize)]
pub struct SyncClubRequest {
    pub club_id: u64,
}

#[derive(Serialize)]
pub struct SyncClubResponse {
    pub members_synced: i32,
    pub activities_imported: i32,
}

pub async fn sync_club(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(req): Json<SyncClubRequest>,
) -> Result<Json<SyncClubResponse>, ApiError> {
    // Get the admin's Strava access token
    let row = sqlx::query!(
        "SELECT strava_access_token FROM users WHERE id = $1",
        admin.0.user_id
    )
    .fetch_one(&state.db)
    .await?;

    let token = row
        .strava_access_token
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("admin has no strava token — re-login first")))?;

    let mut activities_imported = 0i32;
    let mut seen_emails: HashSet<String> = HashSet::new();

    // Fetch up to 10 pages (2 000 most recent club activities)
    'outer: for page in 1..=10u32 {
        let batch = state
            .strava_client
            .get_club_activities(&token, req.club_id, page)
            .await
            .map_err(anyhow::Error::from)?;

        let done = batch.len() < 200;

        for activity in batch {
            if !matches!(activity.activity_type.as_str(), "Run" | "Walk") {
                continue;
            }

            let full_name = format!(
                "{} {}",
                activity.athlete.firstname, activity.athlete.lastname
            );
            // Stable synthetic key — safe to re-run
            let synthetic_email = format!(
                "club-{}-{}@cube.internal",
                activity.athlete.firstname.to_lowercase().replace(' ', "-"),
                activity.athlete.lastname.to_lowercase().replace(' ', "-"),
            );

            seen_emails.insert(synthetic_email.clone());

            // Upsert user (idempotent)
            let user_id = sqlx::query_scalar!(
                "INSERT INTO users (name, email)
                 VALUES ($1, $2)
                 ON CONFLICT (email) DO UPDATE SET name = EXCLUDED.name
                 RETURNING id",
                full_name,
                synthetic_email,
            )
            .fetch_one(&state.db)
            .await?;

            // Club feed has no start_date — use now() so the activity lands in the current week
            let occurred_at = Utc::now();

            // Stable dedup key: athlete slug + type + distance + moving_time
            let external_id = format!(
                "{}-{}-{}-{}",
                synthetic_email.trim_end_matches("@cube.internal"),
                activity.activity_type.to_lowercase(),
                activity.distance as u64,
                activity.moving_time,
            );

            let res = sqlx::query!(
                "INSERT INTO activities
                   (external_id, source, user_id, game_id, activity_type, value, occurred_at)
                 VALUES ($1, 'strava_club', $2, 'walk_run_2026', $3, $4, $5)
                 ON CONFLICT (source, external_id) DO NOTHING",
                external_id,
                user_id,
                activity.activity_type,
                activity.distance / 1000.0,
                occurred_at,
            )
            .execute(&state.db)
            .await?;

            if res.rows_affected() > 0 {
                activities_imported += 1;
            }
        }

        if done {
            break 'outer;
        }
    }

    // Auto-recompute scores for the current week
    let now = Utc::now();
    let year = now.year();
    let week = now.iso_week().week();
    let games = sqlx::query_scalar!("SELECT id FROM games WHERE active = true")
        .fetch_all(&state.db)
        .await?;
    for game_id in games {
        if let Some(scorer) = state.scoring.get(&game_id) {
            let ctx = scoring::ScoringContext::for_week(state.db.clone(), &game_id, year, week);
            scorer.compute_weekly_individual(&ctx).await.map_err(anyhow::Error::from)?;
            scorer.compute_weekly_team(&ctx).await.map_err(anyhow::Error::from)?;
        }
    }

    Ok(Json(SyncClubResponse {
        members_synced: seen_emails.len() as i32,
        activities_imported,
    }))
}

// ── sync-club-members ─────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct SyncMembersResponse {
    pub members_synced: i32,
}

pub async fn sync_club_members(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(req): Json<SyncClubRequest>,
) -> Result<Json<SyncMembersResponse>, ApiError> {
    let row = sqlx::query!(
        "SELECT strava_access_token FROM users WHERE id = $1",
        admin.0.user_id
    )
    .fetch_one(&state.db)
    .await?;

    let token = row
        .strava_access_token
        .ok_or_else(|| ApiError::Internal(anyhow::anyhow!("admin has no strava token")))?;

    let members = state
        .strava_client
        .get_club_members(&token, req.club_id)
        .await
        .map_err(anyhow::Error::from)?;

    let total = members.len() as i32;

    for m in &members {
        let email = m.synthetic_email();
        sqlx::query!(
            "INSERT INTO users (name, email)
             VALUES ($1, $2)
             ON CONFLICT (email) DO UPDATE SET name = EXCLUDED.name",
            m.full_name(),
            email,
        )
        .execute(&state.db)
        .await?;
    }

    Ok(Json(SyncMembersResponse { members_synced: total }))
}

// ── export-members ────────────────────────────────────────────────────────────

pub async fn export_members(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<impl IntoResponse, ApiError> {
    let rows = sqlx::query_scalar!(
        r#"SELECT name AS "name!" FROM users ORDER BY name"#
    )
    .fetch_all(&state.db)
    .await?;

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();

    let header_fmt = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xFF8F6F))
        .set_font_color(Color::RGB(0x000000));

    sheet.set_column_width(0, 30).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
    sheet.set_column_width(1, 16).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
    sheet.set_column_width(2, 26).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;

    sheet.write_with_format(0, 0, "Name",        &header_fmt).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
    sheet.write_with_format(0, 1, "Team Number", &header_fmt).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
    sheet.write_with_format(0, 2, "Team Name",   &header_fmt).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;

    for (i, name) in rows.iter().enumerate() {
        let r = (i + 1) as u32;
        sheet.write(r, 0, name.as_str()).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
        sheet.write(r, 1, "").map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
        sheet.write(r, 2, "").map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;
    }

    let bytes = workbook.save_to_buffer().map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"cube_games_members.xlsx\""),
        ],
        bytes,
    ))
}
