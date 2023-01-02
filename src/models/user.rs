// use crate::errors::Error;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password_hash: String,
    pub email: String,
    pub role_id: i32,
    pub avatar: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub last_seen: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
}

// pub struct CreateUser {
//     pub name: String,
//     pub password: String,
//     pub email: String,
//     pub avatar: Option<String>,
// }

// pub struct UpdateUser {
//     pub name: String,
//     pub email: String,
//     pub avatar: Option<String>,
// }

#[derive(FromRow, Debug)]
pub struct PublicUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    // pub async fn create(pool: &MySqlPool, user_info: &CreateUser) -> Result<PublicUser, Error> {
    //     let row = sqlx::query_as!(
    //         PublicUser,
    //         "INSERT INTO user(name, password_hash, email, avatar, role_id, is_active) VALUES (?, ?, ?, ?, ?, ?)",
    //         user_info.name,
    //         user_info.password,
    //         user_info.email,
    //         user_info.avatar,
    //         1,
    //         0,
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     PublicUser::from_row(&row).map_err(Error::Database)
    // }

    // pub async fn find_by_id(pool: &MySqlPool, id: i32) -> Result<PublicUser, Error> {
    //     let row = sqlx::query_as!(
    //         PublicUser,
    //         "SELECT id, name, email, avatar, created_at, last_seen, deleted_at FROM user WHERE id = ?",
    //         id
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(row)
    // }

    // pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<PublicUser, Error> {
    //     let row = sqlx::query_as!(
    //         PublicUser,
    //         "SELECT id, name, email, avatar, created_at, last_seen, deleted_at FROM user WHERE email = ?",
    //         email
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(row)
    // }

    // pub async fn find_by_name(pool: &MySqlPool, name: &str) -> Result<PublicUser, Error> {
    //     let row = sqlx::query_as!(
    //         PublicUser,
    //         "SELECT id, name, email, avatar, created_at, last_seen, deleted_at FROM user WHERE name = ?",
    //         name
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(row)
    // }
}
