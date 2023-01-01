use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub article_id: String,
    pub user_id: i32,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
