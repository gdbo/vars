use std::net::SocketAddr;
use std::sync::Arc;

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

pub struct AppState {
    pub pool: MySqlPool,
    pub secret: String,
}

pub async fn serve(settings: Settings, pool: MySqlPool) {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);
    let app_state = Arc::new(AppState {
        pool,
        secret: settings.auth.secret,
    });

    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/api", api::create_route())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .with_state(Arc::clone(&app_state))
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
