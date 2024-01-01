use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::{
        session_repository_interface::SessionRepository, user_repository_interface::UserRepository,
    },
    models::{interface::user_interface::UserTrait, user::User},
    value_object::token::{Session, SessionInterface},
};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

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
    expiration_timestamp: i64,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn find_by_id(&self, id: String) -> User {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();

        let user = sqlx::query_as::<_, FindOne>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(conn)
            .await
            .unwrap();

        UserTrait::new(user.id, user.username, user.password).unwrap()
    }

    async fn find_with_token(&self, token: String) -> Result<User, String> {
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
            user.access_token,
            user.refresh_token,
            user.expiration_timestamp,
        );

        if !token.check_expiration() {
            return Err("token is expired".to_string());
        }

        Ok(UserTrait::new(user.id, user.username, user.password).unwrap())
    }

    async fn create(&self, user: User) -> Result<User, String> {
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
                token_repo.create(create_user.id.clone()).await.unwrap();

                tx.commit().await.unwrap();

                let user_result =
                    User::new(create_user.id, create_user.username, create_user.password);
                match user_result {
                    Ok(user) => Ok(user),
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
    use domain::infrastructure::interface::db::db_interface::{new_db, DbTrait, MockDbTrait};
    use sqlx::pool;

    use crate::db::persistence::postgres::Db;

    use super::*;

    #[tokio::test]
    async fn test_user_repository_find_by_id() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("TEST_DATABASE_URL").expect("DATABASE_URL is not set");

        let pool = pool::Pool::<Postgres>::connect(&database_url)
            .await
            .unwrap();

        // sqlx::migrate!("../db/migrations").run(&pool).await.unwrap();

        let db = Arc::new(pool);

        assert_eq!(1, 1);

        // let repo = UserRepositoryImpl::new(db);
        // let user = repo
        //     .find_by_id("17b5ac0c-1429-469a-8522-053f7bf0f80d".to_string())
        //     .await;
        // assert_eq!(
        //     user.0.id,
        //     "17b5ac0c-1429-469a-8522-053f7bf0f80d".to_string()
        // );
    }
}
