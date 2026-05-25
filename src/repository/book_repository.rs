
use async_trait::async_trait;
use sqlx::PgPool;

use crate::model::book::Book;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn find_all(&self) -> anyhow::Result<Vec<Book>>;
    async fn save(&self, book: Book) -> anyhow::Result<Book>;
}

pub struct PostgresBookRepository {
    pool: PgPool,
}

impl PostgresBookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookRepository for PostgresBookRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<Book>> {
        let books = sqlx::query_as::<_, Book>(
            r#"
            SELECT id, title, author, available
            FROM books
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(books)
    }

    async fn save(&self, book: Book) -> anyhow::Result<Book> {
        sqlx::query(
            r#"
            INSERT INTO books(id, title, author, available)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(book.id)
        .bind(&book.title)
        .bind(&book.author)
        .bind(book.available)
        .execute(&self.pool)
        .await?;

        Ok(book)
    }
}
