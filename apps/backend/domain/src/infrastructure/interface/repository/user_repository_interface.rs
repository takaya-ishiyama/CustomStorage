use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{pool::PoolConnection, postgres::PgConnectOptions, Pool, Postgres};

use crate::models::user::User;

use mockall::automock;

#[automock]
#[async_trait]
pub trait UserRepository {
    // FIXME: poolの参照を渡すようにする
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_id(&self, id: String) -> User;
    async fn find_with_token(&self, token: String) -> User;
    async fn create(&self, data: User) -> Result<User, String>;
}
