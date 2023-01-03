use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type AppResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Database(#[from] sqlx::Error),

    // #[error("Failed to read application context")]
    // ReadContext,
    #[error("{0}")]
    Auth(#[from] AuthError),

    #[error("{0} not found")]
    NotFound(String),

    // #[error("{0}")]
    // BadRequest(String),
    #[error("{0}")]
    ObjectConflict(String),

    #[error("hash password")]
    HashPassword(#[from] argon2::password_hash::Error),
}

impl Error {
    pub fn code(&self) -> u32 {
        match self {
            Error::Database(_) => 1001,
            // Error::ReadContext => 1002,
            Error::Auth(_) => 2001,
            Error::NotFound(_) => 2002,
            // Error::BadRequest(_) => 2003,
            Error::ObjectConflict(_) => 2004,
            Error::HashPassword(_) => 2005,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!(
            { "code": self.code(), "message": self.to_string() }
        ));
        (StatusCode::OK, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,
    #[error("Missing authentication credentials")]
    MissingCredentials,
    #[error("Failed to create authentication token")]
    TokenCreation,
    #[error("Invalid authentication token")]
    InvalidToken,
}
