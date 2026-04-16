use chrono::{Datelike, Utc};
use chrono_tz::Asia::Bangkok;
use core::AppConfig;
use scoring::ScoringContext;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::load()?;
    let db = db::create_pool(&config.database.url, config.database.max_connections).await?;
    let registry = Arc::new(scoring::ScoringRegistry::new());

    let sched = JobScheduler::new().await?;

    // Weekly rollup — Sunday 23:59 Bangkok (16:59 UTC)
    {
        let db = db.clone();
        let registry = registry.clone();
        sched
            .add(Job::new_async("0 59 16 * * 0", move |_, _| {
                let db = db.clone();
                let registry = registry.clone();
                Box::pin(async move {
                    let bangkok_now = Utc::now().with_timezone(&Bangkok);
                    let year = bangkok_now.year();
                    let week = bangkok_now.iso_week().week();

                    let games = match sqlx::query_scalar!(
                        "SELECT id FROM games WHERE active = true"
                    )
                    .fetch_all(&db)
                    .await
                    {
                        Ok(g) => g,
                        Err(e) => {
                            tracing::error!("failed to fetch games: {e}");
                            return;
                        }
                    };

                    for game_id in games {
                        if let Some(scorer) = registry.get(&game_id) {
                            let ctx =
                                ScoringContext::for_week(db.clone(), &game_id, year, week);
                            if let Err(e) = scorer.compute_weekly_individual(&ctx).await {
                                tracing::error!(%game_id, "individual rollup failed: {e}");
                            }
                            if let Err(e) = scorer.compute_weekly_team(&ctx).await {
                                tracing::error!(%game_id, "team rollup failed: {e}");
                            }
                        }
                    }
                    tracing::info!(year, week, "weekly rollup complete");
                })
            })?)
            .await?;
    }

    // Token refresh — every hour
    {
        let db = db.clone();
        let strava_oauth = Arc::new(strava::StravaOAuth::new(
            &config.strava.client_id,
            &config.strava.client_secret,
            &config.strava.redirect_uri,
        ));
        sched
            .add(Job::new_async("0 0 * * * *", move |_, _| {
                let db = db.clone();
                let oauth = strava_oauth.clone();
                Box::pin(async move {
                    let now = Utc::now();
                    let expiring = match sqlx::query!(
                        "SELECT id, strava_refresh_token FROM users
                         WHERE strava_token_expires_at < $1
                           AND strava_refresh_token IS NOT NULL",
                        now + chrono::Duration::minutes(10),
                    )
                    .fetch_all(&db)
                    .await
                    {
                        Ok(rows) => rows,
                        Err(e) => {
                            tracing::error!("token refresh query failed: {e}");
                            return;
                        }
                    };

                    for row in expiring {
                        let refresh = match row.strava_refresh_token {
                            Some(r) => r,
                            None => continue,
                        };
                        match oauth.refresh_token(&refresh).await {
                            Ok((access, new_refresh, expires_at)) => {
                                let exp = chrono::DateTime::from_timestamp(expires_at, 0)
                                    .unwrap_or(now);
                                let _ = sqlx::query!(
                                    "UPDATE users SET
                                       strava_access_token = $1,
                                       strava_refresh_token = $2,
                                       strava_token_expires_at = $3
                                     WHERE id = $4",
                                    access,
                                    new_refresh,
                                    exp,
                                    row.id,
                                )
                                .execute(&db)
                                .await;
                            }
                            Err(e) => {
                                tracing::warn!(user_id = row.id, "token refresh failed: {e}");
                            }
                        }
                    }
                })
            })?)
            .await?;
    }

    // Webhook processor — every 30 seconds
    {
        let db = db.clone();
        let strava_client = Arc::new(strava::StravaClient::new());
        sched
            .add(Job::new_async("every 30 seconds", move |_, _| {
                let db = db.clone();
                let client = strava_client.clone();
                Box::pin(async move {
                    process_webhook_queue(&db, &client).await;
                })
            })?)
            .await?;
    }

    sched.start().await?;
    tracing::info!("scheduler running");
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn process_webhook_queue(db: &sqlx::PgPool, client: &strava::StravaClient) {
    let events = match sqlx::query!(
        "SELECT id, payload FROM webhook_events
         WHERE processed = false AND error IS NULL
         ORDER BY received_at
         LIMIT 50"
    )
    .fetch_all(db)
    .await
    {
        Ok(e) => e,
        Err(e) => {
            tracing::error!("webhook queue fetch failed: {e}");
            return;
        }
    };

    for event in events {
        let result = handle_event(db, client, &event.payload).await;
        let (processed, error) = match result {
            Ok(()) => (true, None::<String>),
            Err(e) => {
                tracing::warn!(event_id = event.id, "webhook processing failed: {e}");
                (false, Some(e.to_string()))
            }
        };
        let _ = sqlx::query!(
            "UPDATE webhook_events
             SET processed = $1, error = $2, processed_at = now()
             WHERE id = $3",
            processed,
            error,
            event.id,
        )
        .execute(db)
        .await;
    }
}

async fn handle_event(
    db: &sqlx::PgPool,
    client: &strava::StravaClient,
    payload: &serde_json::Value,
) -> anyhow::Result<()> {
    let owner_id = payload["owner_id"]
        .as_i64()
        .ok_or_else(|| anyhow::anyhow!("missing owner_id"))?;
    let activity_id = payload["object_id"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("missing object_id"))?;

    let user = sqlx::query!(
        "SELECT id, strava_access_token FROM users WHERE strava_athlete_id = $1",
        owner_id,
    )
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow::anyhow!("unknown athlete {owner_id}"))?;

    let token = user
        .strava_access_token
        .ok_or_else(|| anyhow::anyhow!("user {} has no access token", user.id))?;

    let activity = client.get_activity(&token, activity_id).await?;

    // Only process Run / Walk
    if !matches!(activity.activity_type.as_str(), "Run" | "Walk") {
        return Ok(());
    }

    let occurred_at = chrono::DateTime::parse_from_rfc3339(&activity.start_date)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    sqlx::query!(
        "INSERT INTO activities
           (external_id, source, user_id, game_id, activity_type, value, occurred_at, raw_data)
         VALUES ($1, 'strava', $2, 'walk_run_2026', $3, $4, $5, $6)
         ON CONFLICT (source, external_id) DO NOTHING",
        activity_id.to_string(),
        user.id,
        activity.activity_type,
        activity.distance_km(),
        occurred_at,
        serde_json::to_value(&activity).ok(),
    )
    .execute(db)
    .await?;

    Ok(())
}
