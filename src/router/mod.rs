use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;
use sqlx::MySqlPool;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    // settings: Arc<Settings>,
    pub db: MySqlPool,
}

pub async fn serve(settings: Settings, db: MySqlPool) {
    let app = Router::new()
        .route("/ping", get(ping))
        .with_state(AppState { db });

    let addr = SocketAddr::from(([127, 0, 0, 1], settings.server.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn ping() -> &'static str {
    "pong"
}
