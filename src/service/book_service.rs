
use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::book::Book,
    repository::book_repository::BookRepository,
};

#[async_trait]
pub trait BookService: Send + Sync {
    async fn get_books(&self) -> anyhow::Result<Vec<Book>>;
    async fn create_book(
        &self,
        title: String,
        author: String,
    ) -> anyhow::Result<Book>;
}

pub struct DefaultBookService {
    repository: Arc<dyn BookRepository>,
}

impl DefaultBookService {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl BookService for DefaultBookService {
    async fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        self.repository.find_all().await
    }

    async fn create_book(
        &self,
        title: String,
        author: String,
    ) -> anyhow::Result<Book> {

        let book = Book {
            id: Uuid::new_v4(),
            title,
            author,
            available: true,
        };

        self.repository.save(book).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockBookRepository {
        books: Vec<Book>,
    }

    #[async_trait]
    impl BookRepository for MockBookRepository {
        async fn find_all(&self) -> anyhow::Result<Vec<Book>> {
            Ok(self.books.clone())
        }

        async fn save(&self, book: Book) -> anyhow::Result<Book> {
            Ok(book)
        }
    }

    fn make_service(books: Vec<Book>) -> DefaultBookService {
        DefaultBookService::new(Arc::new(MockBookRepository { books }))
    }

    #[tokio::test]
    async fn test_get_books_returns_all() {
        let books = vec![
            Book {
                id: Uuid::new_v4(),
                title: "Rust Programming".to_string(),
                author: "Steve Klabnik".to_string(),
                available: true,
            },
            Book {
                id: Uuid::new_v4(),
                title: "The Pragmatic Programmer".to_string(),
                author: "Hunt & Thomas".to_string(),
                available: false,
            },
        ];
        let service = make_service(books);

        let result = service.get_books().await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "Rust Programming");
        assert_eq!(result[1].title, "The Pragmatic Programmer");
    }

    #[tokio::test]
    async fn test_get_books_returns_empty_list() {
        let service = make_service(vec![]);

        let result = service.get_books().await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_create_book_sets_available_true() {
        let service = make_service(vec![]);

        let book = service
            .create_book("Clean Code".to_string(), "Robert Martin".to_string())
            .await
            .unwrap();

        assert!(book.available);
    }

    #[tokio::test]
    async fn test_create_book_preserves_title_and_author() {
        let service = make_service(vec![]);

        let book = service
            .create_book("Clean Code".to_string(), "Robert Martin".to_string())
            .await
            .unwrap();

        assert_eq!(book.title, "Clean Code");
        assert_eq!(book.author, "Robert Martin");
    }

    #[tokio::test]
    async fn test_create_book_assigns_unique_ids() {
        let service = make_service(vec![]);

        let book1 = service
            .create_book("Book A".to_string(), "Author A".to_string())
            .await
            .unwrap();
        let book2 = service
            .create_book("Book B".to_string(), "Author B".to_string())
            .await
            .unwrap();

        assert_ne!(book1.id, book2.id);
    }
}
