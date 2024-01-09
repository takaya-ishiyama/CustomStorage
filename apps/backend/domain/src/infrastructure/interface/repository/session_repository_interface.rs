use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use crate::value_object::token::Session;

#[automock]
#[async_trait]
pub trait SessionRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn create(&self, user_id: &str) -> Result<Session, String>;
    async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Session, String>;
    async fn update(&self, session: &Session) -> Result<Session, String>;
    async fn upsert(&self, session: &Session) -> Result<Session, String>;
}
