use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub strava: StravaConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StravaConfig {
    pub client_id: String,
    pub client_secret: String,
    pub webhook_verify_token: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub session_duration_hours: i64,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let cfg = config::Config::builder()
            .add_source(config::File::with_name("config/base").required(false))
            .add_source(config::File::with_name("config/local").required(false))
            .add_source(config::Environment::with_prefix("CUBEGAMES").separator("__"))
            // Allow DATABASE_URL directly
            .set_override_option(
                "database.url",
                std::env::var("DATABASE_URL").ok(),
            )?
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}
