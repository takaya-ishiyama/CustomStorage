use std::sync::Arc;

use async_trait::async_trait;
use domain::infrastructure::interface::repository::directory_repository_interface::DirectoriesRepository;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct DirectoriesRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl DirectoriesRepository for DirectoriesRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
}
