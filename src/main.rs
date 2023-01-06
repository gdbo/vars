mod api;
mod database;
mod errors;
mod logger;
mod models;
mod router;
mod settings;
mod utils;

use anyhow::Context;
use dotenvy::dotenv;
use settings::Settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load environment variables from .env file
    dotenv().context(".env file not found")?;

    let settings = Settings::new()?;
    let pool = database::new(&settings.database.url).await?;
    // sqlx::migrate!().run(&pool).await?;
    logger::setup(&settings.logger.level);

    router::serve(settings, pool).await;
    Ok(())
}
