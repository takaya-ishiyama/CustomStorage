use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::models::user::User;

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_id(&self, id: String) -> User;
    async fn create(&self, t: User) -> User;
}
