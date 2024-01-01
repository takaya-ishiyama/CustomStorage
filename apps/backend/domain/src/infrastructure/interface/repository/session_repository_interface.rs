use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use crate::value_object::token::Session;

#[automock]
#[async_trait]
pub trait SessionRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn create(&self, user_id: String) -> Result<Session, String>;
}
