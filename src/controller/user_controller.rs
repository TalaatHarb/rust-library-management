
use axum::{
    extract::{Path, State},
    Json,
};

use uuid::Uuid;

use crate::{
    app_state::AppState,
    model::{
        book::Book,
        borrow::BorrowRecord,
    },
};

#[utoipa::path(
    get,
    path = "/api/books",
    responses(
        (status = 200, body = [Book])
    )
)]
pub async fn get_books(
    State(state): State<AppState>,
) -> Json<Vec<Book>> {

    let books = state
        .book_service
        .get_books()
        .await
        .unwrap();

    Json(books)
}

#[utoipa::path(
    post,
    path = "/api/books/{id}/borrow",
    responses(
        (status = 200, body = BorrowRecord)
    )
)]
pub async fn borrow_book(
    State(state): State<AppState>,
    Path(book_id): Path<Uuid>,
) -> Json<BorrowRecord> {

    let result = state
        .borrow_service
        .borrow_book(Uuid::new_v4(), book_id)
        .await
        .unwrap();

    Json(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
        Router,
    };
    use chrono::Utc;
    use tower::ServiceExt;

    use crate::{
        app_state::AppState,
        config::Config,
        service::{book_service::BookService, borrow_service::BorrowService},
    };

    struct MockBookService {
        books: Vec<Book>,
    }

    #[async_trait]
    impl BookService for MockBookService {
        async fn get_books(&self) -> anyhow::Result<Vec<Book>> {
            Ok(self.books.clone())
        }

        async fn create_book(&self, title: String, author: String) -> anyhow::Result<Book> {
            Ok(Book { id: Uuid::new_v4(), title, author, available: true })
        }
    }

    struct MockBorrowService;

    #[async_trait]
    impl BorrowService for MockBorrowService {
        async fn borrow_book(&self, user_id: Uuid, book_id: Uuid) -> anyhow::Result<BorrowRecord> {
            Ok(BorrowRecord {
                id: Uuid::new_v4(),
                user_id,
                book_id,
                borrowed_at: Utc::now().naive_utc(),
            })
        }
    }

    fn test_state(books: Vec<Book>) -> AppState {
        AppState {
            config: Config {
                port: 3000,
                database_url: String::new(),
                oauth2_schema: "http".to_string(),
                oauth2_host: "localhost".to_string(),
                oauth2_port: 8080,
                oauth2_realm: "master".to_string(),
                oauth2_issuer_url: String::new(),
                oauth2_token_url: String::new(),
            },
            book_service: Arc::new(MockBookService { books }),
            borrow_service: Arc::new(MockBorrowService),
        }
    }

    fn app(books: Vec<Book>) -> Router {
        Router::new()
            .route("/api/books", get(get_books))
            .route("/api/books/:id/borrow", post(borrow_book))
            .with_state(test_state(books))
    }

    #[tokio::test]
    async fn test_get_books_returns_200() {
        let response = app(vec![])
            .oneshot(
                Request::builder()
                    .uri("/api/books")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_books_returns_all_books() {
        let books = vec![
            Book { id: Uuid::new_v4(), title: "Rust".to_string(), author: "Steve".to_string(), available: true },
            Book { id: Uuid::new_v4(), title: "Go".to_string(), author: "Rob".to_string(), available: false },
        ];
        let response = app(books)
            .oneshot(
                Request::builder()
                    .uri("/api/books")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let result: Vec<Book> = serde_json::from_slice(&body).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "Rust");
        assert_eq!(result[1].title, "Go");
    }

    #[tokio::test]
    async fn test_get_books_returns_empty_list() {
        let response = app(vec![])
            .oneshot(
                Request::builder()
                    .uri("/api/books")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let result: Vec<Book> = serde_json::from_slice(&body).unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_borrow_book_returns_200() {
        let book_id = Uuid::new_v4();
        let response = app(vec![])
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/books/{}/borrow", book_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_borrow_book_returns_record_with_correct_book_id() {
        let book_id = Uuid::new_v4();
        let response = app(vec![])
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/api/books/{}/borrow", book_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let record: BorrowRecord = serde_json::from_slice(&body).unwrap();

        assert_eq!(record.book_id, book_id);
    }
}
