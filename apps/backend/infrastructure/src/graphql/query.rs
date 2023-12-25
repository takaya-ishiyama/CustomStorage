use async_graphql::*;
use domain::models::user::User as DomainUser;
use sqlx::{Acquire, FromRow};

use crate::{
    db::persistence::postgres::Db,
    repository::{user_repository::UserRepository, Repository},
};

#[derive(SimpleObject, sqlx::FromRow)]
struct Ping {
    status: String,
    code: i32,
}

#[derive(SimpleObject)]
struct User {
    id: String,
    username: String,
    password: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn ping(&self) -> Ping {
        Ping {
            status: "ok".to_string(),
            code: 200,
        }
    }

    async fn user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        // #[graphql(desc = "Id of object")] id: String,
    ) -> Result<User> {
        let db = ctx.data::<Db>().unwrap().0.clone();
        // let mut pool = ctx.data::<Db>()?.0.acquire().await?;
        // let mut pool = db.acquire().await.unwrap();
        // let conn = pool.acquire().await.unwrap();
        let repo = UserRepository::new(db);
        let user = repo.find_one(1).await;
        let user = User {
            id: user.0.id,
            username: user.1.username,
            password: user.1.password,
        };
        // conn.begin().await?;

        // let users: User = sqlx::query_as::<_, User>("SELECT id,username FROM users")
        //     .fetch_one(conn)
        //     .await?;

        Ok(user)
    }
}
