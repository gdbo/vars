use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub email: String,
    pub role_id: i32,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
    pub is_active: bool,
}
