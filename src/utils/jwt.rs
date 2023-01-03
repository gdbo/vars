use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    errors::{AppResult, AuthError, Error},
    models::user::PublicUser,
    router::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<PublicUser> for AuthToken {
    fn from(value: PublicUser) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        let state = AppState::from_ref(state);

        let token_data = decode(bearer.token(), &state.secret)
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize, // Expiration time (as UTC timestamp). validate_exp defaults to true in validation
    pub iat: usize, // Issued at (as UTC timestamp)
    pub user: AuthToken,
}

impl Claims {
    pub fn new(user: PublicUser) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::days(30)).timestamp() as usize,
            iat: chrono::Local::now().timestamp() as usize,
            user: AuthToken::from(user),
        }
    }
}

pub fn encode(user: PublicUser, secret: &str) -> AppResult<String> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let claims = Claims::new(user);

    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| Error::Auth(AuthError::TokenCreation))
}

pub fn decode(token: &str, secret: &str) -> AppResult<TokenData<Claims>> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    jsonwebtoken::decode(token, &decoding_key, &Validation::default())
        .map_err(|_| Error::Auth(AuthError::InvalidToken))
}
