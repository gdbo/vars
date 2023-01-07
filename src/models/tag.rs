use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
