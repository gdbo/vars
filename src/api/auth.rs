use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    errors::{AppResult, AuthError, Error},
    models::user::User,
    router::AppState,
    utils::{hash::verify_password, jwt},
};

use super::ApiResponse;

pub fn create_route() -> Router<AppState> {
    Router::new().route("/", post(authorize))
}

async fn authorize(
    state: State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> AppResult<Json<Value>> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(Error::Auth(AuthError::MissingCredentials));
    }

    let user = User::find_by_name_or_email(&state.db, &payload.email, &payload.email).await?;
    if user.is_none() {
        return Err(Error::Auth(AuthError::WrongCredentials));
    }

    let user = user.unwrap();
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(Error::Auth(AuthError::WrongCredentials));
    }

    let token = jwt::encode(user, &state.secret)?;

    let res = AuthResponse {
        access_token: token,
    };

    Ok(Json(serde_json::json!(ApiResponse::new(res))))
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}
