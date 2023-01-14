use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde_json::Value;

use super::{ApiResponse, Pagination};
use crate::{
    errors::{AppResult, Error},
    models::article::{Article, CreateArticle, UpdateArticle},
    router::AppState,
    utils::jwt::Claims,
};

pub fn create_route() -> Router<AppState> {
    Router::new()
        .route("/", get(get_articles).post(create_article))
        .route(
            "/:id",
            get(get_article).put(update_article).delete(delete_article),
        )
}

// 注册新标签
pub async fn create_article(
    claims: Claims,
    state: State<AppState>,
    Json(article_info): Json<CreateArticle>,
) -> AppResult<Json<Value>> {
    let user_id = claims.user.id;
    let uid = Article::create(&state.db, user_id, &article_info).await?;
    let new_article = Article::find_by_id(&state.db, uid as i32).await?;
    if new_article.is_none() {
        return Err(Error::NotFound(String::from("article")));
    }

    let new_article = new_article.unwrap();
    let resp = ApiResponse::new(new_article);
    Ok(Json(serde_json::json!(resp)))
}

// 获取标签列表
pub async fn get_articles(
    state: State<AppState>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Value>> {
    let articles = Article::find_list(&state.db, &pagination).await?;

    let resp = ApiResponse::new(articles);
    Ok(Json(serde_json::json!(resp)))
}

// 获取指定标签
pub async fn get_article(state: State<AppState>, Path(id): Path<i32>) -> AppResult<Json<Value>> {
    let article = Article::find_by_id(&state.db, id).await?;
    if article.is_none() {
        return Err(Error::NotFound(String::from("article")));
    }

    let article = article.unwrap();
    let resp = ApiResponse::new(article);
    Ok(Json(serde_json::json!(resp)))
}

// 更新指定标签的信息
pub async fn update_article(
    claims: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(article_info): Json<UpdateArticle>,
) -> AppResult<Json<Value>> {
    let article = Article::find_by_id(&state.db, id).await?;
    if article.is_none() || article.unwrap().user_id != claims.user.id {
        return Err(Error::NotFound(String::from("article")));
    }

    let update_ok = Article::update(&state.db, id, &article_info).await?;
    if !update_ok {
        return Err(Error::NotFound(String::from("article")));
    }

    let article = Article::find_by_id(&state.db, id).await?;
    if article.is_none() {
        return Err(Error::NotFound(String::from("article")));
    }

    let article = article.unwrap();
    let resp = ApiResponse::new(article);
    Ok(Json(serde_json::json!(resp)))
}

// 删除指定标签
pub async fn delete_article(
    claims: Claims,
    state: State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    let article = Article::find_by_id(&state.db, id).await?;
    if article.is_none() || article.unwrap().user_id != claims.user.id {
        return Err(Error::NotFound(String::from("article")));
    }

    Article::delete(&state.db, id).await?;
    let resp = ApiResponse::new(());
    Ok(Json(serde_json::json!(resp)))
}
