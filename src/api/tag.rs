use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde_json::Value;

use super::{ApiResponse, Pagination};
use crate::{
    errors::{AppResult, Error},
    models::tag::{Tag, TagData},
    router::AppState,
    utils::jwt::Claims,
};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_tags).post(create_tag))
        .route("/:id", get(get_tag).put(update_tag).delete(delete_tag))
}

// 注册新标签
pub async fn create_tag(
    State(state): State<Arc<AppState>>,
    Json(tag_info): Json<TagData>,
) -> AppResult<Json<Value>> {
    let exist_tag = Tag::find_by_name(&state.pool, &tag_info.name).await?;
    if exist_tag.is_some() {
        return Err(Error::ObjectConflict(String::from(
            "tagname or email has already been used",
        )));
    }

    let uid = Tag::create(&state.pool, &tag_info).await?;
    let new_tag = Tag::find_by_id(&state.pool, uid as i32).await?;
    if new_tag.is_none() {
        return Err(Error::NotFound(String::from("tag")));
    }

    let new_tag = new_tag.unwrap();
    let resp = ApiResponse::new(new_tag);
    Ok(Json(serde_json::json!(resp)))
}

// 获取标签列表
pub async fn get_tags(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Value>> {
    let tags = Tag::find_list(&state.pool, &pagination).await?;

    let resp = ApiResponse::new(tags);
    Ok(Json(serde_json::json!(resp)))
}

// 获取指定标签
pub async fn get_tag(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    let tag = Tag::find_by_id(&state.pool, id).await?;
    if tag.is_none() {
        return Err(Error::NotFound(String::from("tag")));
    }

    let tag = tag.unwrap();
    let resp = ApiResponse::new(tag);
    Ok(Json(serde_json::json!(resp)))
}

// 更新指定标签的信息
pub async fn update_tag(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(tag_info): Json<TagData>,
) -> AppResult<Json<Value>> {
    let exist_tag = Tag::find_by_name(&state.pool, &tag_info.name).await?;
    if exist_tag.is_some() && exist_tag.unwrap().id != id {
        return Err(Error::ObjectConflict(String::from(
            "tagname or email has already been used",
        )));
    }

    let update_ok = Tag::update(&state.pool, id, &tag_info).await?;
    if !update_ok {
        return Err(Error::NotFound(String::from("tag")));
    }

    let tag = Tag::find_by_id(&state.pool, id).await?;
    if tag.is_none() {
        return Err(Error::NotFound(String::from("tag")));
    }

    let tag = tag.unwrap();
    let resp = ApiResponse::new(tag);
    Ok(Json(serde_json::json!(resp)))
}

// 删除指定标签
pub async fn delete_tag(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    Tag::delete(&state.pool, id).await?;
    let resp = ApiResponse::new(());
    Ok(Json(serde_json::json!(resp)))
}
