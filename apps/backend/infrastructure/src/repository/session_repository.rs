use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::session_repository_interface::SessionRepository,
    value_object::token::{Session, SessionInterface},
};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct SessionRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct CreateToken {
    id: i64,
    user_id: String,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: i64,
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn create(&self, user_id: String) -> Result<Session, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let token = Session::new("".to_string(), "".to_string(), 0);

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
                Ok(Session::new(
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
