use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub summary: String,
    pub cover: String,
    pub status: i8,
    pub password: String,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: bool,
    pub category_id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
}
