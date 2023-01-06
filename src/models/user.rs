use crate::{
    api::{Pagination, PaginationResponse},
    errors::AppResult,
    utils::{avatar::get_avatar_url, hash::generate_hash},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

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

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct PublicUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub async fn create(pool: &MySqlPool, user_info: &CreateUser) -> AppResult<u64> {
        let hash_password = generate_hash(&user_info.password)?;
        let avatar = get_avatar_url(&user_info.email, 64);

        let last_id = sqlx::query_as!(
            PublicUser,
            r#"
                INSERT INTO user(name, password_hash, email, avatar, role_id, is_active)
                VALUES (?, ?, ?, ?, ?, ?);
            "#,
            user_info.name,
            hash_password,
            user_info.email,
            avatar,
            1,
            0,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(last_id)
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<PublicUser>> {
        let row = sqlx::query_as!(
            PublicUser,
            "SELECT id, name, email, password_hash, avatar, created_at, last_seen, deleted_at FROM user WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_name_or_email(
        pool: &MySqlPool,
        name: &str,
        email: &str,
    ) -> AppResult<Option<PublicUser>> {
        let row = sqlx::query_as!(
            PublicUser,
            r#"
                SELECT id, name, email, password_hash, avatar, created_at, last_seen, deleted_at
                FROM user WHERE name = ? OR email = ?;
            "#,
            name,
            email,
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // pub async fn find_by_email(pool: &MySqlPool, email: &str) -> AppResult<Option<PublicUser>> {
    //     let row = sqlx::query_as!(
    //         PublicUser,
    //         "SELECT id, name, email, password_hash, avatar, created_at, last_seen, deleted_at FROM user WHERE email = ?",
    //         email
    //     )
    //     .fetch_optional(pool)
    //     .await?;

    //     Ok(row)
    // }

    // pub async fn find_by_name(pool: &MySqlPool, name: &str) -> AppResult<PublicUser> {
    //     let row = sqlx::query_as!(
    //         PublicUser,
    //         "SELECT id, name, email, avatar, created_at, last_seen, deleted_at FROM user WHERE name = ?",
    //         name
    //     )
    //     .fetch_one(pool)
    //     .await?;

    //     Ok(row)
    // }

    pub async fn find_list(
        pool: &MySqlPool,
        pagination: &Pagination,
    ) -> AppResult<PaginationResponse<PublicUser>> {
        let page = (pagination.page.unwrap_or(1) - 1).max(0);
        let page_size: i32 = pagination.page_size.unwrap_or(10).max(1);

        let rows = sqlx::query_as!(
            PublicUser,
            r#"
                SELECT id, name, email, password_hash, avatar, created_at, last_seen, deleted_at FROM user
                ORDER BY created_at DESC LIMIT ? OFFSET ?;
            "#,
            page_size,
            page * page_size,
        )
        .fetch_all(pool)
        .await?;

        let row = sqlx::query!(r#"SELECT count(*) as total FROM user;"#)
            .fetch_one(pool)
            .await?;

        let pagination = PaginationResponse {
            page: page + 1,
            page_size,
            total: row.total as i32,
            list: rows,
        };

        Ok(pagination)
    }

    pub async fn update(pool: &MySqlPool, id: i32, user_info: &UpdateUser) -> AppResult<bool> {
        let effect_rows = sqlx::query!(
            r#"
                UPDATE user SET
                    name = ?,
                    email = ?,
                    avatar = ?
                WHERE id = ?
            "#,
            user_info.name,
            user_info.email,
            user_info.avatar,
            id,
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(effect_rows == 1)
    }
}
