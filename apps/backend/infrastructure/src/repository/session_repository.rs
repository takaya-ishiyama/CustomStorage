use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::session_repository_interface::SessionRepository,
    value_object::token::{Session, SessionInterface},
};
use sqlx::{
    prelude::FromRow,
    types::chrono::{Local, NaiveDateTime},
    Acquire, Pool, Postgres,
};

#[derive(Clone, Debug)]
pub struct SessionRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct CreateToken {
    // id: i32,
    user_id: String,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn create(&self, user_id: &str) -> Result<Session, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let token = Session::new(user_id, "", "", &Local::now().naive_local()).create();

        let token_result = sqlx::query_as::<_, CreateToken>(
            "INSERT INTO session (user_id, access_token, refresh_token, expiration_timestamp) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(token.user_id)
        .bind(token.access_token)
        .bind(token.refresh_token)
        .bind(token.expiration_timestamp)
        .fetch_one(&mut *tx)
        .await;

        match token_result {
            Ok(token) => {
                tx.commit().await.unwrap();
                Ok(Session::new(
                    &token.user_id,
                    &token.access_token,
                    &token.refresh_token,
                    &token.expiration_timestamp,
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err(e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::test::{setup_testdb::setup_database, test_data::get_test_user};

    use super::*;

    #[sqlx::test]
    async fn test_session_repository_create() -> sqlx::Result<()> {
        let pool = setup_database().await;
        let db = Arc::new(pool);
        let repo = SessionRepositoryImpl::new(db);

        let test_user_id = get_test_user()[0].clone().0.id;

        let session = repo.create(test_user_id.as_str()).await.unwrap();

        assert_eq!(session.user_id, "17b5ac0c-1429-469a-8522-053f7bf0f80d");

        Ok(())
    }
}
