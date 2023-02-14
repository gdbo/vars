mod api;
mod cli;
mod database;
mod errors;
mod logger;
mod models;
mod router;
mod settings;
mod utils;

use anyhow::Context;
use clap::Parser;
use cli::Commands;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    // load environment variables from .env file
    dotenv().context(".env file not found")?;

    let mut settings = settings::init()?;
    let pool = database::init(&settings.database.url).await?;
    logger::init(&settings.logger.level);

    match args.command {
        Some(Commands::Server { port }) => {
            settings.server.port = port.unwrap_or(settings.server.port);
            router::serve(settings, pool).await;
        }
        Some(Commands::Db(_cmd)) => {}
        None => {
            router::serve(settings, pool).await;
        }
    }

    Ok(())
}
