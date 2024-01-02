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

#[derive(Clone, Debug)]
pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct FindOne {
    id: String,
    username: String,
    #[sqlx(skip)]
    password: String,
}

#[derive(FromRow)]
struct Create {
    id: String,
    username: String,
    #[sqlx(skip)]
    password: String,
}

#[derive(FromRow)]
struct FindWithToken {
    id: String,
    username: String,
    #[sqlx(skip)]
    password: String,

    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
}

#[derive(FromRow)]
struct CreateSession {
    // id: i32,
    user_id: String,
    access_token: String,
    refresh_token: String,
    expiration_timestamp: NaiveDateTime,
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
            .bind(id)
            .fetch_one(conn)
            .await
            .unwrap();

        UserTrait::new(&user.id, &user.username, &user.password).unwrap()
    }

    async fn find_with_token(&self, token: &str) -> Result<User, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();
        let user = sqlx::query_as::<_, FindWithToken>(
            "
                SELECT
                    users.user_id,
                    users.username,
                    session.access_token,
                    session.refresh_token,
                    session.expiration_timestamp
                FROM
                    session
                JOIN
                    users ON session.user_id = users.id;
                WHERE
                    session.access_token = $1;
            ",
        )
        .bind(token)
        .fetch_one(conn)
        .await;

        let user = match user {
            Ok(user) => user,
            Err(err) => panic!("{}", "user not found. ".to_string() + &err.to_string()),
        };

        let token = Session::new(
            &user.id,
            &user.access_token,
            &user.refresh_token,
            &user.expiration_timestamp,
        );

        if !token.check_expiration() {
            return Err("token is expired".to_string());
        }

        Ok(UserTrait::new(&user.id, &user.username, &user.password).unwrap())
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
                let session_input_data =
                    Session::new(&create_user.id, "", "", &Local::now().naive_local()).create();

                let session_result = sqlx::query_as::<_, CreateSession>(
                    "INSERT INTO session (user_id, access_token, refresh_token, expiration_timestamp) VALUES ($1, $2, $3, $4) RETURNING *",
                )
                .bind(session_input_data.user_id)
                .bind(session_input_data.access_token)
                .bind(session_input_data.refresh_token)
                .bind(session_input_data.expiration_timestamp)
                .fetch_one(&mut *tx)
                .await;

                let created_session = match session_result {
                    Ok(created_session) => created_session,
                    Err(err) => return Err(err.to_string()),
                };

                tx.commit().await.unwrap();

                let session = Session::new(
                    &created_session.user_id,
                    &created_session.access_token,
                    &created_session.refresh_token,
                    &created_session.expiration_timestamp,
                );

                let user_result = User::new(
                    &create_user.id,
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

    use crate::test::setup_testdb::setup_database;

    use super::*;

    #[sqlx::test]
    async fn test_user_repository_find_by_id() -> sqlx::Result<()> {
        let pool = setup_database().await;
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
    async fn test_user_repository_create() -> sqlx::Result<()> {
        let pool = setup_database().await;
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
}
