pub mod models;
pub mod queries;

use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(url: &str, max_connections: u32) -> anyhow::Result<sqlx::PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await?;
    Ok(pool)
}
