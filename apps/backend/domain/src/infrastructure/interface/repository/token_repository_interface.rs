use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

#[automock]
#[async_trait]
pub trait TokenRepository {
    // FIXME: poolの参照を渡すようにする
    fn new(db: Arc<Pool<Postgres>>) -> Self;
}
