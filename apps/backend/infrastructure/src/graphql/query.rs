use crate::{
    db::persistence::postgres::Db,
    repository::{user_repository::UserRepository, Repository},
};
use async_graphql::*;

#[derive(SimpleObject)]
struct GetUser {
    id: String,
    username: String,
    password: String,
}

pub struct Query;

#[Object]
impl Query {
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
