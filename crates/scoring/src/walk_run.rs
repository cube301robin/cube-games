use super::{GameScoring, ScoringContext};
use async_trait::async_trait;
use tracing::info;

pub struct WalkRunGame;

impl WalkRunGame {
    pub fn new() -> Self {
        Self
    }

    /// Returns (tier_name, bonus_points) for a given weekly km total.
    ///
    /// Tier table (from PRD):
    ///   >= 25km → Elite   (+25 bonus)
    ///   >= 20km → Pacer   (+20 bonus)
    ///   >= 15km → Strider (+15 bonus)
    ///   >= 10km → Runner  (+10 bonus)
    ///   >= 5km  → Mover   (+5 bonus)
    ///   >= 1km  → Starter (+0 bonus)
    ///   < 1km   → Inactive (+0 bonus)
    pub fn tier_for_km(km: f64) -> (&'static str, f64) {
        match km {
            x if x >= 25.0 => ("Elite", 25.0),
            x if x >= 20.0 => ("Pacer", 20.0),
            x if x >= 15.0 => ("Strider", 15.0),
            x if x >= 10.0 => ("Runner", 10.0),
            x if x >= 5.0 => ("Mover", 5.0),
            x if x >= 1.0 => ("Starter", 0.0),
            _ => ("Inactive", 0.0),
        }
    }
}

impl Default for WalkRunGame {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GameScoring for WalkRunGame {
    fn id(&self) -> &'static str {
        "walk_run_2026"
    }

    fn name(&self) -> &'static str {
        "Walk & Run Edition"
    }

    fn validate_activity(&self, activity_type: &str, value: f64) -> anyhow::Result<()> {
        if !matches!(activity_type, "Run" | "Walk") {
            anyhow::bail!("unsupported activity type: {activity_type}");
        }
        if value < 0.0 || value > 1000.0 {
            anyhow::bail!("implausible distance value: {value}");
        }
        Ok(())
    }

    async fn compute_weekly_individual(&self, ctx: &ScoringContext) -> anyhow::Result<usize> {
        let end_exclusive = ctx.end_date + chrono::Duration::days(1);

        let users = sqlx::query!(
            "SELECT id, team_id FROM users WHERE team_id IS NOT NULL"
        )
        .fetch_all(&ctx.db)
        .await?;

        let mut count = 0;
        for user in users {
            let km: f64 = sqlx::query_scalar!(
                r#"SELECT COALESCE(SUM(value), 0.0)::DOUBLE PRECISION
                   FROM activities
                   WHERE user_id = $1
                     AND game_id = $2
                     AND occurred_at >= $3::DATE
                     AND occurred_at <  $4::DATE
                     AND activity_type IN ('Run', 'Walk')"#,
                user.id,
                ctx.game_id,
                ctx.start_date,
                end_exclusive,
            )
            .fetch_one(&ctx.db)
            .await?
            .unwrap_or(0.0);

            let (tier_name, bonus) = Self::tier_for_km(km);
            let total = km + bonus;

            sqlx::query!(
                r#"INSERT INTO weekly_individual_scores
                     (user_id, team_id, game_id, year, iso_week,
                      raw_value, tier_name, base_points, bonus_points, total_points)
                   VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
                   ON CONFLICT (user_id, game_id, year, iso_week) DO UPDATE SET
                     raw_value    = EXCLUDED.raw_value,
                     tier_name    = EXCLUDED.tier_name,
                     base_points  = EXCLUDED.base_points,
                     bonus_points = EXCLUDED.bonus_points,
                     total_points = EXCLUDED.total_points,
                     computed_at  = now()"#,
                user.id,
                user.team_id,
                ctx.game_id,
                ctx.year,
                ctx.iso_week as i32,
                km,
                tier_name,
                km,
                bonus,
                total,
            )
            .execute(&ctx.db)
            .await?;

            count += 1;
        }

        info!(
            game = %ctx.game_id, year = ctx.year, week = ctx.iso_week,
            users = count, "computed individual scores"
        );
        Ok(count)
    }

    async fn compute_weekly_team(&self, ctx: &ScoringContext) -> anyhow::Result<usize> {
        let rows = sqlx::query!(
            r#"INSERT INTO weekly_team_scores
                 (team_id, game_id, year, iso_week,
                  avg_score, active_member_count, total_member_count)
               SELECT
                 t.id,
                 $1,
                 $2,
                 $3,
                 COALESCE(AVG(s.total_points), 0.0)::DOUBLE PRECISION,
                 COUNT(s.id) FILTER (WHERE s.total_points > 0),
                 COUNT(u.id)
               FROM teams t
               LEFT JOIN users u ON u.team_id = t.id
               LEFT JOIN weekly_individual_scores s
                 ON s.user_id = u.id
                 AND s.game_id = $1
                 AND s.year    = $2
                 AND s.iso_week = $3
               GROUP BY t.id
               ON CONFLICT (team_id, game_id, year, iso_week) DO UPDATE SET
                 avg_score           = EXCLUDED.avg_score,
                 active_member_count = EXCLUDED.active_member_count,
                 total_member_count  = EXCLUDED.total_member_count,
                 computed_at         = now()
               RETURNING team_id"#,
            ctx.game_id,
            ctx.year,
            ctx.iso_week as i32,
        )
        .fetch_all(&ctx.db)
        .await?;

        info!(
            game = %ctx.game_id, year = ctx.year, week = ctx.iso_week,
            teams = rows.len(), "computed team scores"
        );
        Ok(rows.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_boundaries() {
        assert_eq!(WalkRunGame::tier_for_km(0.0), ("Inactive", 0.0));
        assert_eq!(WalkRunGame::tier_for_km(0.9), ("Inactive", 0.0));
        assert_eq!(WalkRunGame::tier_for_km(1.0), ("Starter", 0.0));
        assert_eq!(WalkRunGame::tier_for_km(4.9), ("Starter", 0.0));
        assert_eq!(WalkRunGame::tier_for_km(5.0), ("Mover", 5.0));
        assert_eq!(WalkRunGame::tier_for_km(5.3), ("Mover", 5.0));
        assert_eq!(WalkRunGame::tier_for_km(10.0), ("Runner", 10.0));
        assert_eq!(WalkRunGame::tier_for_km(15.0), ("Strider", 15.0));
        assert_eq!(WalkRunGame::tier_for_km(20.0), ("Pacer", 20.0));
        assert_eq!(WalkRunGame::tier_for_km(25.0), ("Elite", 25.0));
        assert_eq!(WalkRunGame::tier_for_km(100.0), ("Elite", 25.0));
    }

    #[test]
    fn total_points_for_5_3_km() {
        let km = 5.3_f64;
        let (tier, bonus) = WalkRunGame::tier_for_km(km);
        assert_eq!(tier, "Mover");
        assert_eq!(bonus, 5.0);
        let total = km + bonus;
        assert!((total - 10.3).abs() < f64::EPSILON);
    }

    #[test]
    fn validate_rejects_unknown_type() {
        let g = WalkRunGame::new();
        assert!(g.validate_activity("Swim", 5.0).is_err());
        assert!(g.validate_activity("Run", 5.0).is_ok());
        assert!(g.validate_activity("Walk", 10.0).is_ok());
    }
}
