use std::sync::Arc;

use async_trait::async_trait;
use domain::infrastructure::interface::repository::items_repository_interface::ItemsRepository;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct ItemsRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl ItemsRepository for ItemsRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
}
