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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::predicate::*;
//     use mockall::*;

//     #[tokio::test]
//     async fn test_database() {
//         // モックの定義
//         mock! {
//             DbConn {}
//             impl DbConn {
//                 async fn get_user(&self, id: i32) -> User;
//             }
//         }

//         let mut db = MockDbConn::new();

//         // モックの振る舞いを定義
//         db.expect_get_user().with(eq(1)).returning(|_| User {
//             id: 1,
//             name: "John".into(),
//         });

//         // テスト対象のコード
//         let user = get_user_from_db(1, &db).await;

//         assert_eq!(user.name, "John");
//     }

//     #[tokio::test]
//     async fn my_async_test() {
//         // Asynchronous test code goes here
//     }

//     #[test]
//     fn my_sync_test() {
//         // Synchronous test code goes here
//     }
// }

// #[derive(Default)]
// pub struct UserQuery;

// #[Object]
// impl UserQuery {
//     async fn get_user(
//         &self,
//         ctx: &Context<'_>,
//         // #[graphql(desc = "id of the post")] id: String,
//     ) -> Result<&String> {
//         // user_service.get_user(id).await.map_err(Into::into);
//         unimplemented!()
//     }
// }

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
