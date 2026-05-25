
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct BorrowRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub borrowed_at: NaiveDateTime,
}
