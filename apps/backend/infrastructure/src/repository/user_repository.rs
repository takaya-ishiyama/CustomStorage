use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::{
        session_repository_interface::SessionRepository, user_repository_interface::UserRepository,
    },
    models::{interface::user_interface::UserTrait, user::User},
    value_object::token::{Session, SessionInterface},
};
use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Local, NaiveDateTime},
    Acquire, Pool, Postgres,
};

use super::session_repository::SessionRepositoryImpl;

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
        let token_repo = SessionRepositoryImpl::new(self.db.clone());

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
                let toke_result = token_repo.create(&create_user.id).await;

                let token = match toke_result {
                    Ok(token) => token,
                    Err(err) => return Err(err),
                };

                tx.commit().await.unwrap();

                let user_result = User::new(
                    &create_user.id,
                    &create_user.username,
                    &create_user.password,
                );
                match user_result {
                    Ok(user) => Ok((user, token)),
                    Err(err) => Err(err),
                }
            }
            Err(err) => {
                tx.rollback().await.unwrap();
                Err(err.to_string())
            }
        }
    }

    // async fn find_all(&self) -> Vec<User> {
    //     let mut pool = self.db.acquire().await.unwrap();
    //     let conn = pool.acquire().await.unwrap();
    //     conn.begin().await.unwrap();
    //     let users = sqlx::query_as!(FindOne, "SELECT * FROM users")
    //         .fetch_all(conn)
    //         .await
    //         .unwrap();
    //     return users;
    // }
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
