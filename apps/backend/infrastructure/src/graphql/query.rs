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
    name: String,
    password: String,
}

// pub struct NewtypeDomainUser(DomainUser);

// impl FromRow<'_, PgRow> for NewtypeDomainUser {
//     fn from_row(row: &PgRow) -> sqlx::Result<Self> {
//         let id: String = row.try_get("id")?;
//         let name: String = row.try_get("name")?;
//         let password: String = row.try_get("password")?;

//         Ok(NewtypeDomainUser(
//             DomainUser::get_user(id, name, password).unwrap(),
//         ))
//     }
// }

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(
        &self,
        ctx: &Context<'_>,
        // #[graphql(desc = "id of the post")] id: String,
    ) -> Result<&String> {
        // user_service.get_user(id).await.map_err(Into::into);
        unimplemented!()
    }
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
// Ok(User {
//     id: users[0].0.id.to_string(),
//     name: users[0].0 .1.name.to_string(),
//     password: users[0].0 .1.password.to_string(),
// })
