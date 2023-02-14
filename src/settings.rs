use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub server: Server,
    pub database: Database,
    pub logger: Logger,
    pub auth: Auth,
}

pub fn init() -> Result<Settings, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

    let mut builder = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("vars"));

    if let Ok(url) = env::var("DATABASE_URL") {
        builder = builder.set_override("database.url", url)?;
    }

    builder
        .build()?
        // Deserialize (and thus freeze) the entire configuration.
        .try_deserialize()
}
