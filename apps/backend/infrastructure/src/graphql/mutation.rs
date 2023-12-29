use async_graphql::{Context, Object, SimpleObject};
use domain::{
    infrastructure::interface::repository::user_repository_interface::UserRepositoryTrait,
    models::{interface::user_interface::UserTrait, user::User},
};

use crate::{db::persistence::postgres::Db, repository::user_repository::UserRepository};

pub struct Mutation;

#[derive(SimpleObject)]
struct CreateUser {
    id: String,
    username: String,
    password: String,
}

#[Object]
impl Mutation {
    async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "name of object")] username: String,
        #[graphql(desc = "password of object")] password: String,
    ) -> Result<CreateUser, String> {
        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = UserRepository::new(db);

        let user = User::new("".to_string(), username, password).unwrap();

        let create_user = repo.create(user).await;

        let user = CreateUser {
            id: create_user.0.id,
            username: create_user.1.username,
            password: create_user.1.password,
        };
        Ok(user)
    }
}
