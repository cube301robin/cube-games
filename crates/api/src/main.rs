mod error;
mod auth;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use cube_core::AppConfig;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub scoring: Arc<scoring::ScoringRegistry>,
    pub strava_client: Arc<strava::StravaClient>,
    pub strava_oauth: Arc<strava::StravaOAuth>,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::load()?;

    let db = db::create_pool(&config.database.url, config.database.max_connections).await?;
    sqlx::migrate!("../../migrations").run(&db).await?;

    let scoring = Arc::new(scoring::ScoringRegistry::new());
    let strava_client = Arc::new(strava::StravaClient::new());
    let strava_oauth = Arc::new(strava::StravaOAuth::new(
        &config.strava.client_id,
        &config.strava.client_secret,
        &config.strava.redirect_uri,
    ));

    let state = AppState {
        db,
        scoring,
        strava_client,
        strava_oauth,
        config: config.clone(),
    };

    let api = Router::new()
        // Health
        .route("/api/health", get(|| async { "ok" }))
        // Auth
        .route("/api/auth/strava/login", get(handlers::auth::strava_login))
        .route("/api/auth/strava/callback", get(handlers::auth::strava_callback))
        // Scoreboard
        .route("/api/scoreboard/individuals", get(handlers::scoreboard::individual_leaderboard))
        .route("/api/scoreboard/teams", get(handlers::scoreboard::team_leaderboard))
        // Strava webhooks
        .route("/api/webhook/strava", get(handlers::webhook::strava_verify))
        .route("/api/webhook/strava", post(handlers::webhook::strava_event))
        // Admin
        .route("/api/admin/recompute", post(handlers::admin::recompute))
        .route("/api/admin/adjustment", post(handlers::admin::add_adjustment))
        .route("/api/admin/sync-club", post(handlers::admin::sync_club))
        .route("/api/admin/sync-club-members", post(handlers::admin::sync_club_members))
        .route("/api/admin/export-members", get(handlers::admin::export_members))
        .with_state(state);

    // Serve static HTML frontend from the `frontend/` directory
    let frontend = ServeDir::new("frontend").append_index_html_on_directories(true);

    let app = Router::new()
        .merge(api)
        .fallback_service(frontend)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
