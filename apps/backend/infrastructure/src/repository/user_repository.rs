use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::user_repository_interface::UserRepository,
    models::{interface::user_interface::UserTrait, user::User},
    value_object::token::{Session, SessionInterface},
};
use sqlx::{
    prelude::FromRow,
    types::chrono::{Local, NaiveDateTime},
    Acquire, Pool, Postgres,
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct FindOne {
    id: Uuid,
    username: String,
    #[sqlx(skip)]
    password: String,
}

#[derive(FromRow)]
struct Create {
    id: Uuid,
    username: String,
    #[sqlx(skip)]
    password: String,
}

#[derive(FromRow)]
struct FindWithToken {
    id: Uuid,
    username: String,
    #[sqlx(skip)]
    password: String,

    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
    expiration_timestamp_for_refresh: NaiveDateTime,
}

#[derive(FromRow)]
struct CreateSession {
    // id: i32,
    user_id: Uuid,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
    expiration_timestamp_for_refresh: NaiveDateTime,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn find_by_id(&self, id: &str) -> User {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();

        let user = sqlx::query_as::<_, FindOne>("SELECT * FROM users WHERE id = $1")
            .bind(Uuid::parse_str(id).unwrap())
            .fetch_one(conn)
            .await
            .unwrap();

        UserTrait::new(&user.id.to_string(), &user.username, &user.password).unwrap()
    }

    async fn find_by_username_and_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<User, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();

        let user = sqlx::query_as::<_, FindOne>(
            "SELECT * FROM users WHERE username = $1 AND password = $2",
        )
        .bind(username)
        .bind(password)
        .fetch_one(conn)
        .await;

        let user = match user {
            Ok(user) => user,
            Err(err) => return Err(err.to_string()),
        };

        Ok(UserTrait::new(&user.id.to_string(), &user.username, &user.password).unwrap())
    }

    async fn find_with_token(&self, token: &str) -> Result<User, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();
        let user = sqlx::query_as::<_, FindWithToken>(
            "
                SELECT
                    users.id,
                    users.username,
                    session.access_token,
                    session.refresh_token,
                    session.expiration_timestamp,
                    session.expiration_timestamp_for_refresh
                FROM
                    session
                JOIN
                    users ON session.user_id = users.id
                WHERE
                    session.access_token = $1;
            ",
        )
        .bind(token)
        .fetch_one(conn)
        .await;

        let user_with_session = match user {
            Ok(user) => user,
            Err(err) => panic!("{}", "user not found. ".to_string() + &err.to_string()),
        };

        let token = Session::new(
            &user_with_session.id.to_string(),
            &user_with_session.access_token,
            &user_with_session.refresh_token,
            &user_with_session.expiration_timestamp,
            &user_with_session.expiration_timestamp_for_refresh,
        );

        if !token.check_expiration() {
            return Err("token is expired".to_string());
        }

        Ok(UserTrait::new(
            &user_with_session.id.to_string(),
            &user_with_session.username,
            &user_with_session.password,
        )
        .unwrap())
    }

    async fn create(&self, user: User) -> Result<(User, Session), String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let create_user_result = sqlx::query_as::<_, Create>(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *",
        )
        .bind(user.1.username)
        .bind(user.1.password)
        .fetch_one(&mut *tx)
        .await;

        match create_user_result {
            Ok(create_user) => {
                let session_input_data = Session::new(
                    &create_user.id.to_string(),
                    "",
                    "",
                    &Local::now().naive_local(),
                    &Local::now().naive_local(),
                )
                .create();

                let session_result = sqlx::query_as::<_, CreateSession>(
                    "INSERT INTO session (user_id, access_token, refresh_token, expiration_timestamp, expiration_timestamp_for_refresh) VALUES ($1, $2, $3, $4, $5) RETURNING *",
                )
                .bind(Uuid::parse_str(&session_input_data.user_id).unwrap())
                .bind(session_input_data.access_token)
                .bind(session_input_data.refresh_token)
                .bind(session_input_data.expiration_timestamp)
                .bind(session_input_data.expiration_timestamp_for_refresh)
                .fetch_one(&mut *tx)
                .await;

                let created_session = match session_result {
                    Ok(created_session) => created_session,
                    Err(err) => return Err(err.to_string()),
                };

                tx.commit().await.unwrap();

                let session = Session::new(
                    &created_session.user_id.to_string(),
                    &created_session.access_token,
                    &created_session.refresh_token,
                    &created_session.expiration_timestamp,
                    &created_session.expiration_timestamp_for_refresh,
                );

                let user_result = User::new(
                    &create_user.id.to_string(),
                    &create_user.username,
                    &create_user.password,
                );
                match user_result {
                    Ok(user) => Ok((user, session)),
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                tx.rollback().await.unwrap();
                Err(err.to_string())
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
    async fn test_user_repository_find_by_id(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = UserRepositoryImpl::new(db);

        let user = repo
            .find_by_id("17b5ac0c-1429-469a-8522-053f7bf0f80d")
            .await;

        assert_eq!(
            user.0.id,
            "17b5ac0c-1429-469a-8522-053f7bf0f80d".to_string()
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_user_repository_find_by_username_and_password(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = UserRepositoryImpl::new(db);

        let init_user = &get_test_user()[0];

        let user = repo
            .find_by_username_and_password(&init_user.1.username, &init_user.1.password)
            .await
            .unwrap();

        assert_eq!(user.0.id, init_user.0.id);

        Ok(())
    }

    #[sqlx::test]
    async fn test_user_repository_create(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = UserRepositoryImpl::new(db);

        let data = User::new("", "test_user_repository_create", "password").unwrap();

        let user = repo.create(data).await.unwrap();

        assert_eq!(
            user.0 .1.username,
            "test_user_repository_create".to_string()
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_user_repository_find_with_token(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = UserRepositoryImpl::new(db);

        let data = User::new("", "test_user", "password").unwrap();
        let create_user = repo.create(data).await.unwrap();

        let user = repo
            .find_with_token(&create_user.1.access_token)
            .await
            .unwrap();

        assert_eq!(user.0.id, create_user.0 .0.id);

        Ok(())
    }
}
