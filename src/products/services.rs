use crate::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use http::StatusCode;
use uuid::Uuid;

use super::models::{CreateProduct, Product, UpdateProduct};

pub async fn get_all_products(state: State<AppState>) -> impl IntoResponse {
    let products = state.products.lock().await;
    return (StatusCode::OK, Json(products.clone())).into_response();
}

pub async fn get_single_product(
    state: State<AppState>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    let products = state.products.lock().await;
    let product = products.iter().find(|p| p.uuid == uuid);
    match product {
        Some(product) => (StatusCode::OK, Json(product.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, "Product not found").into_response(),
    }
}

pub async fn create_product(
    state: State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> impl IntoResponse {
    let mut products = state.products.lock().await;
    let new_product = Product {
        uuid: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,

        price: payload.price,
        stock: payload.stock,
    };

    products.push(new_product.clone());
    return (StatusCode::CREATED, Json(new_product)).into_response();
}

pub async fn update_user(
    state: State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateProduct>,
) -> impl IntoResponse {
    let mut products = state.products.lock().await;
    let product = match products.iter_mut().find(|p| p.uuid == uuid) {
        Some(product) => product,
        None => return (StatusCode::NOT_FOUND, "Product not found").into_response(),
    };

    match payload.name {
        Some(name) => product.name = name.clone(),
        None => (),
    }
    match payload.description {
        Some(description) => product.description = description.clone(),
        None => (),
    }
    match payload.price {
        Some(price) => product.price = price,
        None => (),
    }
    match payload.stock {
        Some(stock) => product.stock = stock,
        None => (),
    }

    return (StatusCode::OK, Json(product.clone())).into_response();
}

pub async fn delete_product(state: State<AppState>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let mut products = state.products.lock().await;
    let index = products.iter().position(|p| p.uuid == uuid);
    match index {
        Some(index) => {
            products.remove(index);
            (StatusCode::NO_CONTENT, "").into_response()
        }
        None => (StatusCode::NOT_FOUND, "Product not found").into_response(),
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_products))
        .with_state(state.clone())
        .route("/:uuid", get(get_single_product))
        .with_state(state.clone())
        .route("/", post(create_product))
        .with_state(state.clone())
        .route("/:uuid", patch(update_user))
        .with_state(state.clone())
        .route("/:uuid", delete(delete_product))
        .with_state(state.clone())
}
