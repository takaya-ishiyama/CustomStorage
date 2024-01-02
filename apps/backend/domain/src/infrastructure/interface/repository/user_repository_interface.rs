use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{models::user::User, value_object::token::Session};

use mockall::automock;

#[automock]
#[async_trait]
pub trait UserRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_id(&self, id: &str) -> User;
    async fn find_with_token(&self, token: &str) -> Result<User, String>;
    async fn find_by_username_and_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, String>;
    async fn create(&self, data: User) -> Result<(User, Session), String>;
}
