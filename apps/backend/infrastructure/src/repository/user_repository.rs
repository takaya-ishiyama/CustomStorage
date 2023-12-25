use std::sync::Arc;

use async_trait::async_trait;
use domain::models::{interface::user_interface::UserTrait, user::User};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

use super::Repository;

pub struct UserRepository {
    //　TODO: ジェネリクスなconnectionを受け取るようにする
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct FindOne {
    id: String,
    username: String,
    #[sqlx(skip)]
    password: String,
}

#[async_trait]
impl Repository<User> for UserRepository {
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
        return User::new(user.id, user.username, user.password).unwrap();
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
