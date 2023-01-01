mod database;
mod models;
mod router;
mod settings;

use anyhow::Context;
use dotenvy::dotenv;
use settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // load environment variables from .env file
    dotenv().context(".env file not found")?;

    let settings = Settings::new()?;
    let pool = database::new(&settings.database.url).await?;
    // sqlx::migrate!().run(&pool).await?;

    router::serve(settings, pool).await;

    Ok(())
}
