use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub article_id: String,
    pub user_id: i32,
    pub like_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
