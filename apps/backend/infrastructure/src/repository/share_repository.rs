use std::sync::Arc;

use async_trait::async_trait;
use domain::infrastructure::interface::repository::share_repository_interface::ShareRepository;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct ShareRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl ShareRepository for ShareRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
}
