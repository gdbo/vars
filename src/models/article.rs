use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

use crate::{
    api::{Pagination, PaginationResponse},
    errors::AppResult,
};

#[derive(FromRow)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: i8,
    pub password: Option<String>,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: bool,
    pub category_id: i32,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: i8,
    pub password: Option<String>,
    pub category_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticle {
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: i8,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: bool,
    pub password: Option<String>,
    pub category_id: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PublicArticle {
    pub id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub cover: Option<String>,
    pub status: i8,
    pub read_count: i32,
    pub like_count: i32,
    pub is_top: i8,
    pub category_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Article {
    pub async fn create(pool: &MySqlPool, author_id: i32, data: &CreateArticle) -> AppResult<u64> {
        let last_id = sqlx::query_as!(
            PublicTag,
            r#"
                INSERT INTO article(title, slug, content, summary, cover, status, password, category_id, user_id)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
            "#,
            data.title,
            data.slug,
            data.content,
            data.summary,
            data.cover,
            data.status,
            data.password,
            data.category_id,
            author_id,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(last_id)
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<PublicArticle>> {
        let row = sqlx::query_as!(
            PublicArticle,
            "SELECT id, title, slug, content, summary, cover, status, read_count, like_count, is_top, category_id, user_id, created_at, updated_at FROM article WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_list(
        pool: &MySqlPool,
        pagination: &Pagination,
    ) -> AppResult<PaginationResponse<PublicArticle>> {
        let page = (pagination.page.unwrap_or(1) - 1).max(0);
        let page_size: i32 = pagination.page_size.unwrap_or(10).max(1);

        let rows = sqlx::query_as!(
            PublicArticle,
            r#"
                SELECT id, title, slug, content, summary, cover, status, read_count, like_count, is_top, category_id, user_id, created_at, updated_at FROM article
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

    pub async fn update(pool: &MySqlPool, id: i32, data: &UpdateArticle) -> AppResult<bool> {
        let effect_rows = sqlx::query!(
            r#"
                UPDATE article SET
                    title = ?,
                    slug = ?,
                    content = ?,
                    summary = ?,
                    cover = ?,
                    status = ?,
                    read_count = ?,
                    like_count = ?,
                    is_top = ?,
                    password = ?,
                    category_id = ?
                WHERE id = ?
            "#,
            data.title,
            data.slug,
            data.content,
            data.summary,
            data.cover,
            data.status,
            data.read_count,
            data.like_count,
            data.is_top,
            data.password,
            data.category_id,
            id,
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(effect_rows == 1)
    }

    pub async fn delete(pool: &MySqlPool, id: i32) -> AppResult<()> {
        sqlx::query!(
            r#"
                delete from article where id = ?
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected();
        Ok(())
    }
}
