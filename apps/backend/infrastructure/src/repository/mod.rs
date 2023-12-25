use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub mod user_repository;

#[async_trait]
pub trait Repository<T> {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_id(&self, id: String) -> T;
    // async fn find_all(&self) -> Vec<T>;
    // async fn save(&self, t: T) -> T;
    // async fn update(&self, t: T) -> T;
    // async fn upsert(&self, t: T) -> T;
    // async fn delete(&self, id: i32) -> T;
}
