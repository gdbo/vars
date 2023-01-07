use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

use crate::{
    api::{Pagination, PaginationResponse},
    errors::AppResult,
};

#[derive(FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryData {
    pub name: String,
    pub description: Option<String>,
}

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct PublicCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Category {
    pub async fn create(pool: &MySqlPool, data: &CategoryData) -> AppResult<u64> {
        let last_id = sqlx::query_as!(
            PublicCategory,
            r#"
                INSERT INTO category(name, description)
                VALUES (?, ?);
            "#,
            data.name,
            data.description,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(last_id)
    }

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> AppResult<Option<PublicCategory>> {
        let row = sqlx::query_as!(
            PublicCategory,
            "SELECT id, name, description, created_at, updated_at FROM category WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_name(pool: &MySqlPool, name: &str) -> AppResult<Option<PublicCategory>> {
        let row = sqlx::query_as!(
            PublicCategory,
            "SELECT id, name, description, created_at, updated_at FROM category WHERE name = ?",
            name
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn find_list(
        pool: &MySqlPool,
        pagination: &Pagination,
    ) -> AppResult<PaginationResponse<PublicCategory>> {
        let page = (pagination.page.unwrap_or(1) - 1).max(0);
        let page_size: i32 = pagination.page_size.unwrap_or(10).max(1);

        let rows = sqlx::query_as!(
            PublicCategory,
            r#"
                SELECT id, name, description, created_at, updated_at FROM category
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

    pub async fn update(pool: &MySqlPool, id: i32, data: &CategoryData) -> AppResult<bool> {
        let effect_rows = sqlx::query!(
            r#"
                UPDATE category SET
                    name = ?,
                    description = ?
                WHERE id = ?
            "#,
            data.name,
            data.description,
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
                delete from category where id = ?
            "#,
            id
        )
        .execute(pool)
        .await?
        .rows_affected();
        Ok(())
    }
}
