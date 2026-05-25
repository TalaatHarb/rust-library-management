
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, sqlx::FromRow)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub available: bool,
}
