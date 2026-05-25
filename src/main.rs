
mod app_state;
mod config;
mod controller;
mod middleware;
mod model;
mod repository;
mod service;

use std::sync::Arc;

use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    app_state::AppState,
    config::Config,
    controller::{admin_controller::*, user_controller::*},
    repository::{
        book_repository::PostgresBookRepository,
        borrow_repository::PostgresBorrowRepository,
    },
    service::{
        book_service::{BookService, DefaultBookService},
        borrow_service::{BorrowService, DefaultBorrowService},
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_books,
        add_book,
        update_book,
        delete_book,
        borrow_book
    ),
    components(
        schemas(
            model::book::Book,
            model::borrow::BorrowRecord,
            controller::admin_controller::CreateBookRequest
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Database connection failed");

    let book_repository = Arc::new(PostgresBookRepository::new(pool.clone()));
    let borrow_repository = Arc::new(PostgresBorrowRepository::new(pool.clone()));

    let book_service: Arc<dyn BookService> =
        Arc::new(DefaultBookService::new(book_repository));

    let borrow_service: Arc<dyn BorrowService> =
        Arc::new(DefaultBorrowService::new(borrow_repository));

    let state = AppState {
        config: config.clone(),
        book_service,
        borrow_service,
    };

    let app = Router::new()
        .route("/api/books", get(get_books))
        .route("/api/books/:id/borrow", post(borrow_book))
        .route("/api/admin/books", post(add_book))
        .route("/api/admin/books/:id", put(update_book))
        .route("/api/admin/books/:id", delete(delete_book))
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .layer(from_fn(middleware::auth_middleware::auth_middleware))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    println!("Running on http://localhost:{}", config.port);
    println!("Swagger at http://localhost:{}/docs", config.port);
    println!("OAuth2 issuer: {}", config.oauth2_issuer_url);
    println!("OAuth2 token:  {}", config.oauth2_token_url);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
