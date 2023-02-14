use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};
use serde_json::Value;

use super::{ApiResponse, Pagination};
use crate::{
    errors::{AppResult, Error},
    models::user::{CreateUser, UpdateUser, User},
    router::AppState,
    utils::jwt::Claims,
};

pub fn create_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/profile", get(get_user_profile))
        .route("/edit", put(edit_user_profile))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

// 注册新用户
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user_info): Json<CreateUser>,
) -> AppResult<Json<Value>> {
    let exist_user =
        User::find_by_name_or_email(&state.pool, &user_info.name, &user_info.email).await?;
    if exist_user.is_some() {
        return Err(Error::ObjectConflict(String::from(
            "username or email has already been used",
        )));
    }

    let uid = User::create(&state.pool, &user_info).await?;
    let new_user = User::find_by_id(&state.pool, uid as i32).await?;
    if new_user.is_none() {
        return Err(Error::NotFound(String::from("user")));
    }

    let new_user = new_user.unwrap();
    let resp = ApiResponse::new(new_user);
    Ok(Json(serde_json::json!(resp)))
}

// 获取用户列表
pub async fn get_users(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> AppResult<Json<Value>> {
    let users = User::find_list(&state.pool, &pagination).await?;

    let resp = ApiResponse::new(users);
    Ok(Json(serde_json::json!(resp)))
}

// 获取指定用户
pub async fn get_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    let user = User::find_by_id(&state.pool, id).await?;
    if user.is_none() {
        return Err(Error::NotFound(String::from("user")));
    }

    let user = user.unwrap();
    let resp = ApiResponse::new(user);
    Ok(Json(serde_json::json!(resp)))
}

// 更新指定用户的信息
pub async fn update_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(user_info): Json<UpdateUser>,
) -> AppResult<Json<Value>> {
    let exist_user =
        User::find_by_name_or_email(&state.pool, &user_info.name, &user_info.email).await?;
    if exist_user.is_some() && exist_user.unwrap().id != id {
        return Err(Error::ObjectConflict(String::from(
            "username or email has already been used",
        )));
    }

    let update_ok = User::update(&state.pool, id, &user_info).await?;
    if !update_ok {
        return Err(Error::NotFound(String::from("user")));
    }

    let user = User::find_by_id(&state.pool, id).await?;
    if user.is_none() {
        return Err(Error::NotFound(String::from("user")));
    }

    let user = user.unwrap();
    let resp = ApiResponse::new(user);
    Ok(Json(serde_json::json!(resp)))
}

// 删除指定用户
pub async fn delete_user(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> AppResult<Json<Value>> {
    User::delete(&state.pool, id).await?;
    let resp = ApiResponse::new(());
    Ok(Json(serde_json::json!(resp)))
}

// 获取当前用户信息
pub async fn get_user_profile(
    claims: Claims,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Value>> {
    let user = User::find_by_id(&state.pool, claims.user.id).await?;
    if user.is_none() {
        return Err(Error::NotFound(String::from("user")));
    }
    let user = user.unwrap();
    let resp = ApiResponse::new(user);
    Ok(Json(serde_json::json!(resp)))
}

// 编辑当前用户信息
pub async fn edit_user_profile(
    claims: Claims,
    State(state): State<Arc<AppState>>,
    Json(user_info): Json<UpdateUser>,
) -> AppResult<Json<Value>> {
    let exist_user = User::find_by_id(&state.pool, claims.user.id).await?;
    if exist_user.is_some() && exist_user.unwrap().id != claims.user.id {
        return Err(Error::ObjectConflict(String::from(
            "username or email has already been used",
        )));
    }

    let update_ok = User::update(&state.pool, claims.user.id, &user_info).await?;
    if !update_ok {
        return Err(Error::NotFound(String::from("user")));
    }

    let user = User::find_by_id(&state.pool, claims.user.id).await?;
    if user.is_none() {
        return Err(Error::NotFound(String::from("user")));
    }

    let user = user.unwrap();
    let resp = ApiResponse::new(user);
    Ok(Json(serde_json::json!(resp)))
}
