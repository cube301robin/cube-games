pub mod walk_run;

use async_trait::async_trait;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ScoringContext {
    pub db: sqlx::PgPool,
    pub game_id: String,
    pub year: i32,
    pub iso_week: u32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl ScoringContext {
    pub fn for_week(db: sqlx::PgPool, game_id: &str, year: i32, week: u32) -> Self {
        let date = NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Mon)
            .expect("invalid iso week");
        Self {
            db,
            game_id: game_id.to_string(),
            year,
            iso_week: week,
            start_date: date,
            end_date: date + chrono::Duration::days(6),
        }
    }
}

#[async_trait]
pub trait GameScoring: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;

    async fn compute_weekly_individual(&self, ctx: &ScoringContext) -> anyhow::Result<usize>;
    async fn compute_weekly_team(&self, ctx: &ScoringContext) -> anyhow::Result<usize>;

    fn validate_activity(&self, _activity_type: &str, _value: f64) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct ScoringRegistry {
    games: HashMap<String, Arc<dyn GameScoring>>,
}

impl ScoringRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            games: HashMap::new(),
        };
        reg.register(Arc::new(walk_run::WalkRunGame::new()));
        reg
    }

    pub fn register(&mut self, game: Arc<dyn GameScoring>) {
        self.games.insert(game.id().to_string(), game);
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn GameScoring>> {
        self.games.get(id).cloned()
    }

    pub fn all_active(&self) -> Vec<Arc<dyn GameScoring>> {
        self.games.values().cloned().collect()
    }
}

impl Default for ScoringRegistry {
    fn default() -> Self {
        Self::new()
    }
}
