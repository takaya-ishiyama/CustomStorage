use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::repository_interface::Repository,
    models::{interface::user_interface::UserTrait, user::User},
};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

pub struct UserRepository {
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

    async fn create(&self, user: User) -> User {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();

        let create_user = sqlx::query_as::<_, Create>(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING *",
        )
        .bind(user.1.username)
        .bind(user.1.password)
        .fetch_one(conn)
        .await
        .unwrap();

        return User::new(create_user.id, create_user.username, create_user.password).unwrap();
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
