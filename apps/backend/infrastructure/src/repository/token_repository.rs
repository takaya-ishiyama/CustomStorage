use std::sync::Arc;

use async_trait::async_trait;
use domain::infrastructure::interface::repository::token_repository_interface::TokenRepository;
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct TokenRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct CreateToken {
    user_id: String,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: i64,
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
}
