use std::sync::Arc;

use async_trait::async_trait;
use domain::models::{interface::user_interface::new_user, user::User};
use sqlx::{prelude::FromRow, Acquire, Pool, Postgres};

use super::Repository;

pub struct UserRepository {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct QueryUser {
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
    async fn find_one(&self, id: String) -> User {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        conn.begin().await.unwrap();
        let user = sqlx::query_as::<_, QueryUser>("SELECT * FROM users")
            .fetch_one(conn)
            .await
            .unwrap();
        return new_user::<User>(user.id, user.username, user.password);
    }

    // fn find_all(&self) -> Vec<DomainUser> {
    //     let users = sqlx::query_as::<_, DomainUser>("SELECT * FROM users")
    //         .fetch_all(self.conn)
    //         .await
    //         .unwrap();
    //     users
    // }

    // fn save(&self, t: DomainUser) -> DomainUser {
    //     let user = sqlx::query_as::<_, DomainUser>("SELECT * FROM users")
    //         .fetch_one(self.conn)
    //         .await
    //         .unwrap();
    //     user
    // }

    // fn update(&self, t: DomainUser) -> DomainUser {
    //     let user = sqlx::query_as::<_, DomainUser>("SELECT * FROM users")
    //         .fetch_one(self.conn)
    //         .await
    //         .unwrap();
    //     user
    // }

    // fn upsert(&self, t: DomainUser) -> DomainUser {
    //     let user = sqlx::query_as::<_, DomainUser>("SELECT * FROM users")
    //         .fetch_one(self.conn)
    //         .await
    //         .unwrap();
    //     user
    // }

    // fn delete(&self, id: i32) -> DomainUser {
    //     let user = sqlx::query_as::<_, DomainUser>("SELECT * FROM users")
    //         .fetch_one(self.conn)
    //         .await
    //         .unwrap();
    //     user
    // }
}
