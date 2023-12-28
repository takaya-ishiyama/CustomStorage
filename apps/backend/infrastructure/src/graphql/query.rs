use crate::{db::persistence::postgres::Db, repository::user_repository::UserRepository};
use async_graphql::*;
use domain::infrastructure::interface::repository::repository_interface::Repository;

#[derive(SimpleObject)]
struct GetUser {
    id: String,
    username: String,
    password: String,
}

pub struct Query;

pub struct Token(pub String);

#[Object]
impl Query {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
    async fn get_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "Id of object")] id: String,
    ) -> Result<GetUser> {
        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = UserRepository::new(db);
        let user = repo.find_by_id(id).await;
        let user = GetUser {
            id: user.0.id,
            username: user.1.username,
            password: user.1.password,
        };
        Ok(user)
    }
}
