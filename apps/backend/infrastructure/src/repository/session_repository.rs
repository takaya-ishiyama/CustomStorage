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
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct CreateToken {
    user_id: Uuid,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
    expiration_timestamp_for_refresh: NaiveDateTime,
}

#[derive(FromRow)]
struct GetAccessTokenByRefreshToken {
    user_id: Uuid,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
    expiration_timestamp_for_refresh: NaiveDateTime,
}

#[derive(FromRow)]
struct FindByRefreshToken {
    user_id: Uuid,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
    expiration_timestamp_for_refresh: NaiveDateTime,
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    /** 使ってない。使うときは、user_idが一致する場合は、updateするようにするかエラーを返すようにする */
    async fn create(&self, user_id: &str) -> Result<Session, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let token = Session::new(
            user_id,
            "",
            "",
            &Local::now().naive_local(),
            &Local::now().naive_local(),
        )
        .create();

        let token_result = sqlx::query_as::<_, CreateToken>(
            "INSERT INTO session (user_id, access_token, refresh_token, expiration_timestamp, expiration_timestamp_for_refresh) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(Uuid::parse_str(&token.user_id).unwrap())
        .bind(token.access_token)
        .bind(token.refresh_token)
        .bind(token.expiration_timestamp)
        .bind(token.expiration_timestamp_for_refresh)
        .fetch_one(&mut *tx)
        .await;

        match token_result {
            Ok(token) => {
                tx.commit().await.unwrap();
                Ok(Session::new(
                    &token.user_id.to_string(),
                    &token.access_token,
                    &token.refresh_token,
                    &token.expiration_timestamp,
                    &token.expiration_timestamp_for_refresh,
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err(e.to_string())
            }
        }
    }

    async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Session, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let session_result = sqlx::query_as::<_, FindByRefreshToken>(
            "SELECT * FROM session WHERE refresh_token = $1",
        )
        .bind(refresh_token)
        .fetch_one(&mut *tx)
        .await;

        match session_result {
            Ok(_session) => {
                let session = Session::new(
                    &_session.user_id.to_string(),
                    &_session.access_token,
                    &_session.refresh_token,
                    &_session.expiration_timestamp,
                    &_session.expiration_timestamp_for_refresh,
                );
                if !session.check_expiration_for_refresh() {
                    return Err("refresh token is expired".to_string());
                }

                tx.commit().await.unwrap();
                Ok(session)
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err("session not found: ".to_string() + &e.to_string())
            }
        }
    }

    async fn update(&self, session: &Session) -> Result<Session, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let session_result = sqlx::query_as::<_, GetAccessTokenByRefreshToken>(
            "UPDATE session SET access_token = $1, refresh_token = $2, expiration_timestamp = $3 WHERE user_id = $4 RETURNING *",
        )
        .bind(&session.access_token)
        .bind(&session.refresh_token)
        .bind(session.expiration_timestamp)
        .bind(Uuid::parse_str(&session.user_id).unwrap())
        .fetch_one(&mut *tx)
        .await;

        match session_result {
            Ok(session) => {
                tx.commit().await.unwrap();
                Ok(Session::new(
                    &session.user_id.to_string(),
                    &session.access_token,
                    &session.refresh_token,
                    &session.expiration_timestamp,
                    &session.expiration_timestamp_for_refresh,
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err("session not found: ".to_string() + &e.to_string())
            }
        }
    }

    async fn upsert(&self, session: &Session) -> Result<Session, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();

        let mut tx = conn.begin().await.unwrap();

        let session_result = sqlx::query_as::<_, GetAccessTokenByRefreshToken>(
            "
            INSERT INTO session (user_id, access_token, refresh_token, expiration_timestamp, expiration_timestamp_for_refresh) 
            VALUES ($1, $2, $3, $4, $5) 
            ON CONFLICT (user_id) 
            DO UPDATE SET access_token = $2, refresh_token = $3, expiration_timestamp = $4, expiration_timestamp_for_refresh = $5 
            RETURNING *
            ",
        )
        .bind(Uuid::parse_str(&session.user_id).unwrap())
        .bind(&session.access_token)
        .bind(&session.refresh_token)
        .bind(session.expiration_timestamp)
        .bind(session.expiration_timestamp_for_refresh)
        .fetch_one(&mut *tx)
        .await;

        match session_result {
            Ok(session) => {
                tx.commit().await.unwrap();
                Ok(Session::new(
                    &session.user_id.to_string(),
                    &session.access_token,
                    &session.refresh_token,
                    &session.expiration_timestamp,
                    &session.expiration_timestamp_for_refresh,
                ))
            }
            Err(e) => {
                tx.rollback().await.unwrap();
                Err("session not found: ".to_string() + &e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use sqlx::PgPool;

    use crate::test::{setup_testdb::setup_database, test_data::get_test_user};

    use super::*;

    #[sqlx::test]
    async fn test_session_repository_create(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = SessionRepositoryImpl::new(db);

        let test_user_id = get_test_user()[0].clone().0.id;

        let session = repo.create(test_user_id.as_str()).await.unwrap();

        assert_eq!(session.user_id, "17b5ac0c-1429-469a-8522-053f7bf0f80d");

        Ok(())
    }

    #[sqlx::test]
    async fn test_session_repository_get_access_token(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = SessionRepositoryImpl::new(db);

        let test_user_id = get_test_user()[0].clone().0.id;

        let create_session = repo.create(test_user_id.as_str()).await.unwrap();

        let session = repo
            .find_by_refresh_token(create_session.refresh_token.as_str())
            .await
            .unwrap();

        assert_eq!(session.access_token, create_session.access_token);

        Ok(())
    }

    #[sqlx::test]
    async fn test_session_repository_update(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = SessionRepositoryImpl::new(db);

        let test_user_id = get_test_user()[0].clone().0.id;

        let create_session = repo.create(test_user_id.as_str()).await.unwrap();

        let input_session = Session::new(
            &create_session.user_id,
            "",
            "",
            &Local::now().naive_local(),
            &Local::now().naive_local(),
        )
        .create();

        let session = repo
            .update(&Session::new(
                &create_session.user_id,
                &input_session.access_token,
                &create_session.refresh_token,
                &input_session.expiration_timestamp,
                &input_session.expiration_timestamp_for_refresh,
            ))
            .await
            .unwrap();

        let exp_diff = session.expiration_timestamp - input_session.expiration_timestamp;

        assert_eq!(session.access_token, input_session.access_token);
        assert_eq!(session.refresh_token, create_session.refresh_token);
        assert!(exp_diff.num_seconds() < 1);

        Ok(())
    }

    #[sqlx::test]
    async fn test_session_repository_upsert_create(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = SessionRepositoryImpl::new(db);

        let test_user_id = get_test_user()[0].clone().0.id;

        let create_session = repo.create(test_user_id.as_str()).await.unwrap();

        let input_session = Session::new(
            &create_session.user_id,
            "",
            "",
            &Local::now().naive_local(),
            &Local::now().naive_local(),
        )
        .create();

        let session = repo
            .upsert(&Session::new(
                &create_session.user_id,
                &input_session.access_token,
                &create_session.refresh_token,
                &input_session.expiration_timestamp,
                &input_session.expiration_timestamp_for_refresh,
            ))
            .await
            .unwrap();

        let exp_diff = session.expiration_timestamp - input_session.expiration_timestamp;

        assert_eq!(session.access_token, input_session.access_token);
        assert_eq!(session.refresh_token, create_session.refresh_token);
        assert!(exp_diff.num_seconds() < 1);

        Ok(())
    }

    #[sqlx::test]
    async fn test_session_repository_upsert_update(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = SessionRepositoryImpl::new(db);

        let test_user_id = get_test_user()[0].clone().0.id;

        let input_session = Session::new(
            &test_user_id,
            "",
            "",
            &Local::now().naive_local(),
            &Local::now().naive_local(),
        )
        .create();

        let session = repo
            .upsert(&Session::new(
                &test_user_id,
                &input_session.access_token,
                &input_session.refresh_token,
                &input_session.expiration_timestamp,
                &input_session.expiration_timestamp_for_refresh,
            ))
            .await
            .unwrap();

        let exp_diff = session.expiration_timestamp - input_session.expiration_timestamp;

        assert_eq!(session.user_id, test_user_id);
        assert_eq!(session.access_token, input_session.access_token);
        assert_eq!(session.refresh_token, input_session.refresh_token);
        assert!(exp_diff.num_seconds() < 1);

        Ok(())
    }
}
