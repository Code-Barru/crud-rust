use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Product {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub stock: i32,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub stock: i32,
}

#[derive(Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub stock: Option<i32>,
}
