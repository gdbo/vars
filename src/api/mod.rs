use axum::Router;
use serde::{Deserialize, Serialize};

use crate::router::AppState;

pub mod auth;
pub mod category;
pub mod tag;
pub mod user;

pub fn create_route() -> Router<AppState> {
    Router::new()
        .nest("/users", user::create_route())
        .nest("/categories", category::create_route())
        .nest("/tags", tag::create_route())
        .nest("/auth", auth::create_route())
}

#[derive(Serialize, Debug)]
pub struct ApiResponse<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Default for ApiResponse<T> {
    fn default() -> Self {
        ApiResponse {
            code: 0,
            message: String::from("success"),
            data: None,
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> ApiResponse<T> {
        ApiResponse {
            code: 0,
            message: String::from("success"),
            data: Some(data),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub page: i32,
    pub page_size: i32,
    pub total: i32,
    pub list: Vec<T>,
}
