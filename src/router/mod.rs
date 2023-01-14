use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use sqlx::MySqlPool;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::api;
use crate::errors::{AppResult, Error};
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    // settings: Arc<Settings>,
    pub db: MySqlPool,
    pub secret: String,
}

pub async fn serve(settings: Settings, db: MySqlPool) {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/api", api::create_route())
        .with_state(AppState {
            db,
            secret: settings.auth.secret,
        })
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .fallback(handler_404);

    let addr = SocketAddr::from(([127, 0, 0, 1], settings.server.port));
    tracing::info!("Listening on http://127.0.0.1:{}", settings.server.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn ping() -> &'static str {
    "pong"
}

async fn handler_404() -> (StatusCode, AppResult<()>) {
    (
        StatusCode::NOT_FOUND,
        Err(Error::NotFound(String::from(""))),
    )
}
