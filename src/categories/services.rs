use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use http::StatusCode;
use uuid::Uuid;

use super::models::{Category, CreateCategory, UpdateCategory};

pub async fn get_all_categories(state: State<AppState>) -> impl IntoResponse {
    let categories = state.categories.lock().await;
    return (StatusCode::OK, Json(categories.clone())).into_response();
}

pub async fn get_single_category(
    state: State<AppState>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let categories = state.categories.lock().await;
    let category = categories.iter().find(|c| c.uuid == uuid);
    match category {
        Some(category) => (StatusCode::OK, Json(category.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "Category not found").into_response(),
    }
}

pub async fn create_category(
    state: State<AppState>,
    Json(payload): Json<CreateCategory>,
) -> impl IntoResponse {
    let mut categories = state.categories.lock().await;
    let new_category = Category {
        uuid: Uuid::new_v4(),
        name: payload.name,
        color: payload.color,
    };

    categories.push(new_category.clone());
    return (StatusCode::CREATED, Json(new_category)).into_response();
}

pub async fn update_category(
    state: State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateCategory>,
) -> impl IntoResponse {
    let mut categories = state.categories.lock().await;
    let category = match categories.iter_mut().find(|c| c.uuid == uuid) {
        Some(category) => category,
        None => return (StatusCode::NOT_FOUND, "Category not found").into_response(),
    };

    match payload.name {
        Some(name) => category.name = name.clone(),
        None => (),
    }

    match payload.color {
        Some(color) => category.color = color,
        None => (),
    }

    return (StatusCode::OK, Json(category.clone())).into_response();
}

pub async fn delete_category(state: State<AppState>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let mut categories = state.categories.lock().await;
    let index = categories.iter().position(|c| c.uuid == uuid);
    match index {
        Some(index) => {
            categories.remove(index);
            (StatusCode::NO_CONTENT, "").into_response()
        }
        None => (StatusCode::NOT_FOUND, "Category not found").into_response(),
    }
}

pub fn get_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/", get(get_all_categories))
        .route("/:uuid", get(get_single_category))
        .route("/", post(create_category))
        .route("/:uuid", put(update_category))
        .route("/:uuid", delete(delete_category))
        .with_state(state.clone())
}
