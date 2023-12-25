use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub mod user_repository;

#[async_trait]
pub trait Repository<T> {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_one(&self, id: String) -> T;
    // async fn find_all(&self) -> Vec<T>;
    // async fn save(&self, t: T) -> T;
    // async fn update(&self, t: T) -> T;
    // async fn upsert(&self, t: T) -> T;
    // async fn delete(&self, id: i32) -> T;
}

// #[async_trait]
// trait RepoTrait {
//     async fn get_connection(&mut self) -> Result<&mut PgConnection>;
// }

// pub struct Repo {
//     pool: PoolConnection<Postgres>,
// }

// #[async_trait]
// impl RepoTrait for Repo {
//     async fn get_connection(&mut self) -> Result<&mut PgConnection> {
//         let conn = self.pool.acquire().await?;
//         Ok(conn)
//     }
// }
