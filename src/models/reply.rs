use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Reply {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub comment_id: String,
    pub reply_id: String,
    pub reply_type: bool,
    pub like_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
