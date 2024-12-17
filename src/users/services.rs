use crate::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use http::StatusCode;
use uuid::Uuid;

use super::models::{CreateUser, UpdateUser, User};

pub async fn get_all_users(state: State<AppState>) -> impl IntoResponse {
    let users = state.users.lock().await;
    return (StatusCode::OK, Json(users.clone())).into_response();
}

pub async fn get_single_user(state: State<AppState>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let users = state.users.lock().await;
    let user = users.iter().find(|u| u.uuid == uuid);
    match user {
        Some(user) => (StatusCode::OK, Json(user.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}

pub async fn create_user(
    state: State<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    let new_user = User {
        uuid: uuid::Uuid::new_v4(),
        username: payload.username,
        email: payload.email,
        password: payload.password,
    };

    users.push(new_user.clone());
    return (StatusCode::CREATED, Json(new_user)).into_response();
}

pub async fn update_user(
    state: State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    let user = match users.iter_mut().find(|u| u.uuid == uuid) {
        Some(user) => user,
        None => return (StatusCode::NOT_FOUND, "User not found").into_response(),
    };

    match payload.username {
        Some(username) => user.username = username.clone(),
        None => (),
    }
    match payload.email {
        Some(email) => user.email = email.clone(),
        None => (),
    }
    match payload.password {
        Some(password) => user.password = password.clone(),
        None => (),
    }

    (StatusCode::OK, Json(user.clone())).into_response()
}

pub async fn delete_user(state: State<AppState>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let mut users = state.users.lock().await;
    let user_index = users.iter().position(|u| u.uuid == uuid);
    match user_index {
        Some(index) => {
            users.remove(index);
            (StatusCode::NO_CONTENT, "User deleted").into_response()
        }
        None => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_users))
        .with_state(state.clone())
        .route("/:uuid", get(get_single_user))
        .with_state(state.clone())
        .route("/", post(create_user))
        .with_state(state.clone())
        .route("/:uuid", put(update_user))
        .with_state(state.clone())
        .route("/:uuid", delete(delete_user))
        .with_state(state.clone())
}
