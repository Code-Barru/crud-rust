use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Category {
    pub uuid: Uuid,
    pub name: String,
    pub color: u8,
}

#[derive(Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub color: u8,
}

#[derive(Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub color: Option<u8>,
}
