
use std::sync::Arc;

use crate::{
    config::Config,
    service::{
        book_service::BookService,
        borrow_service::BorrowService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub book_service: Arc<dyn BookService>,
    pub borrow_service: Arc<dyn BorrowService>,
}
