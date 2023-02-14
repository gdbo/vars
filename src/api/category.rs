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
    models::category::{Category, CategoryData},
    router::AppState,
    utils::jwt::Claims,
};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/:id",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
}

// 注册新分类
pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(category_info): Json<CategoryData>,
) -> AppResult<Json<Value>> {
    let exist_category = Category::find_by_name(&state.pool, &category_info.name).await?;
    if exist_category.is_some() {
        return Err(Error::ObjectConflict(String::from(
            "categoryname or email has already been used",
        )));
    }

    let uid = Category::create(&state.pool, &category_info).await?;
    let new_category = Category::find_by_id(&state.pool, uid as i32).await?;
    if new_category.is_none() {
        return Err(Error::NotFound(String::from("category")));
    }

    let new_category = new_category.unwrap();
    let resp = ApiResponse::new(new_category);
    Ok(Json(serde_json::json!(resp)))
}

// 获取分类列表
pub async fn get_categories(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Value>> {
    let categories = Category::find_list(&state.pool, &pagination).await?;

    let resp = ApiResponse::new(categories);
    Ok(Json(serde_json::json!(resp)))
}

// 获取指定分类
pub async fn get_category(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    let category = Category::find_by_id(&state.pool, id).await?;
    if category.is_none() {
        return Err(Error::NotFound(String::from("category")));
    }

    let category = category.unwrap();
    let resp = ApiResponse::new(category);
    Ok(Json(serde_json::json!(resp)))
}

// 更新指定分类的信息
pub async fn update_category(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(category_info): Json<CategoryData>,
) -> AppResult<Json<Value>> {
    let exist_category = Category::find_by_name(&state.pool, &category_info.name).await?;
    if exist_category.is_some() && exist_category.unwrap().id != id {
        return Err(Error::ObjectConflict(String::from(
            "categoryname or email has already been used",
        )));
    }

    let update_ok = Category::update(&state.pool, id, &category_info).await?;
    if !update_ok {
        return Err(Error::NotFound(String::from("category")));
    }

    let category = Category::find_by_id(&state.pool, id).await?;
    if category.is_none() {
        return Err(Error::NotFound(String::from("category")));
    }

    let category = category.unwrap();
    let resp = ApiResponse::new(category);
    Ok(Json(serde_json::json!(resp)))
}

// 删除指定分类
pub async fn delete_category(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    Category::delete(&state.pool, id).await?;
    let resp = ApiResponse::new(());
    Ok(Json(serde_json::json!(resp)))
}
