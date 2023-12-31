use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::token_repository_interface::TokenRepository,
    value_object::token::{Token, TokenInterface},
};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

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
    async fn create(&self, user_id: String) -> Result<Token, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let token = Token::new("".to_string(), "".to_string(), 0);

        let token_result = sqlx::query_as::<_, CreateToken>(
            "INSERT INTO session (user_id, access_token, refresh_token, expiration_timestamp) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(user_id)
        .bind(token.access_token)
        .bind(token.refresh_token)
        .bind(0)
        .fetch_one(&mut *tx)
        .await;

        match token_result {
            Ok(token) => {
                tx.commit().await.unwrap();
                Ok(Token::new(
                    token.access_token,
                    token.refresh_token,
                    token.expiration_timestamp,
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err(e.to_string())
            }
        }
    }
}
