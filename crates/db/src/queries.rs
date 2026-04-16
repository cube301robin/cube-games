use crate::models::{IndividualLeaderboardEntry, TeamLeaderboardEntry, User};
use sqlx::PgPool;

pub async fn get_user_by_strava_id(db: &PgPool, athlete_id: i64) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        "SELECT id, uuid, name, email, team_id, strava_athlete_id, is_admin, created_at
         FROM users WHERE strava_athlete_id = $1",
        athlete_id
    )
    .fetch_optional(db)
    .await
}

pub async fn upsert_strava_user(
    db: &PgPool,
    athlete_id: i64,
    name: &str,
    email: Option<&str>,
    access_token: &str,
    refresh_token: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        r#"INSERT INTO users (name, email, strava_athlete_id, strava_access_token, strava_refresh_token, strava_token_expires_at)
           VALUES ($1, $2, $3, $4, $5, $6)
           ON CONFLICT (strava_athlete_id) DO UPDATE SET
             name = EXCLUDED.name,
             email = COALESCE(EXCLUDED.email, users.email),
             strava_access_token = EXCLUDED.strava_access_token,
             strava_refresh_token = EXCLUDED.strava_refresh_token,
             strava_token_expires_at = EXCLUDED.strava_token_expires_at
           RETURNING id, uuid, name, email, team_id, strava_athlete_id, is_admin, created_at"#,
        name,
        email,
        athlete_id,
        access_token,
        refresh_token,
        expires_at,
    )
    .fetch_one(db)
    .await
}

pub async fn get_individual_leaderboard(
    db: &PgPool,
    game_id: &str,
    year: i32,
    iso_week: i32,
    limit: i64,
    offset: i64,
) -> sqlx::Result<Vec<IndividualLeaderboardEntry>> {
    sqlx::query_as!(
        IndividualLeaderboardEntry,
        r#"SELECT
             ROW_NUMBER() OVER (ORDER BY s.total_points DESC) AS "rank!",
             u.id AS user_id,
             u.name AS user_name,
             t.name AS team_name,
             s.tier_name,
             s.raw_value,
             s.total_points
           FROM weekly_individual_scores s
           JOIN users u ON u.id = s.user_id
           LEFT JOIN teams t ON t.id = s.team_id
           WHERE s.game_id = $1 AND s.year = $2 AND s.iso_week = $3
           ORDER BY s.total_points DESC
           LIMIT $4 OFFSET $5"#,
        game_id,
        year,
        iso_week,
        limit,
        offset,
    )
    .fetch_all(db)
    .await
}

pub async fn get_team_leaderboard(
    db: &PgPool,
    game_id: &str,
    year: i32,
    iso_week: i32,
) -> sqlx::Result<Vec<TeamLeaderboardEntry>> {
    sqlx::query_as!(
        TeamLeaderboardEntry,
        r#"SELECT
             ROW_NUMBER() OVER (ORDER BY w.avg_score DESC) AS "rank!",
             t.id AS team_id,
             t.name AS team_name,
             w.avg_score,
             w.active_member_count,
             w.total_member_count
           FROM weekly_team_scores w
           JOIN teams t ON t.id = w.team_id
           WHERE w.game_id = $1 AND w.year = $2 AND w.iso_week = $3
           ORDER BY w.avg_score DESC"#,
        game_id,
        year,
        iso_week,
    )
    .fetch_all(db)
    .await
}

pub async fn get_user_weekly_score(
    db: &PgPool,
    user_id: i32,
    game_id: &str,
    year: i32,
    iso_week: i32,
) -> sqlx::Result<Option<crate::models::WeeklyIndividualScore>> {
    sqlx::query_as!(
        crate::models::WeeklyIndividualScore,
        "SELECT id, user_id, team_id, game_id, year, iso_week, raw_value, tier_name,
                base_points, bonus_points, total_points
         FROM weekly_individual_scores
         WHERE user_id = $1 AND game_id = $2 AND year = $3 AND iso_week = $4",
        user_id,
        game_id,
        year,
        iso_week,
    )
    .fetch_optional(db)
    .await
}
