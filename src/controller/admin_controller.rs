
use axum::{
    extract::{Path, State},
    Json,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{app_state::AppState, model::book::Book};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
}

#[utoipa::path(
    post,
    path = "/api/admin/books",
    request_body = CreateBookRequest,
    responses(
        (status = 200, body = Book)
    )
)]
pub async fn add_book(
    State(state): State<AppState>,
    Json(request): Json<CreateBookRequest>,
) -> Json<Book> {

    let result = state
        .book_service
        .create_book(request.title, request.author)
        .await
        .unwrap();

    Json(result)
}

#[utoipa::path(
    put,
    path = "/api/admin/books/{id}",
    responses(
        (status = 200)
    )
)]
pub async fn update_book(
    Path(_id): Path<Uuid>,
) -> &'static str {
    "TODO update"
}

#[utoipa::path(
    delete,
    path = "/api/admin/books/{id}",
    responses(
        (status = 200)
    )
)]
pub async fn delete_book(
    Path(_id): Path<Uuid>,
) -> &'static str {
    "TODO delete"
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use chrono::Utc;
    use tower::ServiceExt;

    use crate::{
        app_state::AppState,
        config::Config,
        model::borrow::BorrowRecord,
        service::{book_service::BookService, borrow_service::BorrowService},
    };

    struct MockBookService;

    #[async_trait]
    impl BookService for MockBookService {
        async fn get_books(&self) -> anyhow::Result<Vec<Book>> {
            Ok(vec![])
        }

        async fn create_book(&self, title: String, author: String) -> anyhow::Result<Book> {
            Ok(Book {
                id: Uuid::new_v4(),
                title,
                author,
                available: true,
            })
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

    fn test_state() -> AppState {
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
            book_service: Arc::new(MockBookService),
            borrow_service: Arc::new(MockBorrowService),
        }
    }

    fn app() -> Router {
        Router::new()
            .route("/api/admin/books", post(add_book))
            .with_state(test_state())
    }

    #[tokio::test]
    async fn test_add_book_returns_200() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/books")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"Rust Book","author":"Steve"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_add_book_returns_book_with_correct_fields() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/books")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"Clean Code","author":"Robert Martin"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let book: Book = serde_json::from_slice(&body).unwrap();

        assert_eq!(book.title, "Clean Code");
        assert_eq!(book.author, "Robert Martin");
        assert!(book.available);
    }

    #[tokio::test]
    async fn test_add_book_returns_422_for_missing_fields() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/admin/books")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"No Author"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
