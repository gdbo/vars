use anyhow::Context;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn init(url: &str) -> anyhow::Result<MySqlPool> {
    MySqlPoolOptions::new()
        .max_connections(50)
        .connect(url)
        .await
        .context("could not connect to database_url")
}
