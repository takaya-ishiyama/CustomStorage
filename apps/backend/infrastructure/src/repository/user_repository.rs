use std::sync::Arc;

use async_trait::async_trait;
use axum::Error;
use domain::{
    infrastructure::interface::repository::user_repository_interface::UserRepository,
    models::{interface::user_interface::UserTrait, user::User},
};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

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

    async fn find_with_token(&self, token: String) -> User {
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
                    users
                JOIN
                    session ON users.id = session.user_id;
        ",
        )
        .bind(token)
        .fetch_one(conn)
        .await
        .unwrap();
        return UserTrait::new(user.id, user.username, user.password).unwrap();
    }

    async fn create(&self, user: User) -> Result<User, String> {
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

// #[cfg(test)]
// mod tests {
//     use domain::infrastructure::interface::db::db_interface::{DbTrait, MockDbTrait};

//     use super::*;

//     #[tokio::test]
//     async fn test_user_repository_find_by_id() {
//         let mut db_mock = MockDbTrait::new();

//         let repo = UserRepository::new(db_mock);
//         let user = repo.find_by_id("1".to_string()).await;
//         assert_eq!(user.0.id, "1".to_string());
//     }
// }
