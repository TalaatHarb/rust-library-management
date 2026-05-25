
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::borrow::BorrowRecord,
    repository::borrow_repository::BorrowRepository,
};

#[async_trait]
pub trait BorrowService: Send + Sync {
    async fn borrow_book(
        &self,
        user_id: Uuid,
        book_id: Uuid,
    ) -> anyhow::Result<BorrowRecord>;
}

pub struct DefaultBorrowService {
    repository: Arc<dyn BorrowRepository>,
}

impl DefaultBorrowService {
    pub fn new(repository: Arc<dyn BorrowRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl BorrowService for DefaultBorrowService {
    async fn borrow_book(
        &self,
        user_id: Uuid,
        book_id: Uuid,
    ) -> anyhow::Result<BorrowRecord> {
        self.repository.borrow_book(user_id, book_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Utc;

    struct MockBorrowRepository;

    #[async_trait]
    impl BorrowRepository for MockBorrowRepository {
        async fn borrow_book(
            &self,
            user_id: Uuid,
            book_id: Uuid,
        ) -> anyhow::Result<BorrowRecord> {
            Ok(BorrowRecord {
                id: Uuid::new_v4(),
                user_id,
                book_id,
                borrowed_at: Utc::now().naive_utc(),
            })
        }
    }

    #[tokio::test]
    async fn test_borrow_book_returns_correct_user_and_book_ids() {
        let service = DefaultBorrowService::new(Arc::new(MockBorrowRepository));
        let user_id = Uuid::new_v4();
        let book_id = Uuid::new_v4();

        let record = service.borrow_book(user_id, book_id).await.unwrap();

        assert_eq!(record.user_id, user_id);
        assert_eq!(record.book_id, book_id);
    }

    #[tokio::test]
    async fn test_borrow_book_assigns_unique_record_ids() {
        let service = DefaultBorrowService::new(Arc::new(MockBorrowRepository));

        let r1 = service.borrow_book(Uuid::new_v4(), Uuid::new_v4()).await.unwrap();
        let r2 = service.borrow_book(Uuid::new_v4(), Uuid::new_v4()).await.unwrap();

        assert_ne!(r1.id, r2.id);
    }

    #[tokio::test]
    async fn test_borrow_book_sets_borrowed_at() {
        let service = DefaultBorrowService::new(Arc::new(MockBorrowRepository));
        let before = Utc::now().naive_utc();

        let record = service.borrow_book(Uuid::new_v4(), Uuid::new_v4()).await.unwrap();

        let after = Utc::now().naive_utc();
        assert!(record.borrowed_at >= before && record.borrowed_at <= after);
    }
}
