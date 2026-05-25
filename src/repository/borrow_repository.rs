
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::borrow::BorrowRecord;

#[async_trait]
pub trait BorrowRepository: Send + Sync {
    async fn borrow_book(
        &self,
        user_id: Uuid,
        book_id: Uuid,
    ) -> anyhow::Result<BorrowRecord>;
}

pub struct PostgresBorrowRepository {
    pool: PgPool,
}

impl PostgresBorrowRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BorrowRepository for PostgresBorrowRepository {
    async fn borrow_book(
        &self,
        user_id: Uuid,
        book_id: Uuid,
    ) -> anyhow::Result<BorrowRecord> {

        let record = BorrowRecord {
            id: Uuid::new_v4(),
            user_id,
            book_id,
            borrowed_at: Utc::now().naive_utc(),
        };

        Ok(record)
    }
}
