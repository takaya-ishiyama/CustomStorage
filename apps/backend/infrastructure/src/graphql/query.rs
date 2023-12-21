// use async_graphql::{Context, Object, Result, Scalar, SimpleObject};
use async_graphql::*;
use sqlx::{postgres::PgRow, Acquire, FromRow, Row};

use crate::db::persistence::postgres::Db;
use domain::models::user::User as DomainUser;
use uuid::Uuid;

#[derive(SimpleObject, sqlx::FromRow)]
struct Ping {
    status: String,
    code: i32,
}

#[derive(SimpleObject, FromRow)]
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

    async fn users<'ctx>(&self, ctx: &Context<'ctx>) -> Result<User> {
        let mut pool = ctx.data::<Db>()?.0.acquire().await?;
        let conn = pool.acquire().await?;
        conn.begin().await?;

        let users: User = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_one(conn)
            .await?;

        Ok(users)
    }
}
