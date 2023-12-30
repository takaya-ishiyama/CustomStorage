use async_graphql::{Context, Object, SimpleObject};
use domain::{
    infrastructure::interface::repository::user_repository_interface::UserRepository,
    models::{interface::user_interface::UserTrait, user::User},
};

use crate::{db::persistence::postgres::Db, repository::user_repository::UserRepositoryImpl};

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
        let repo = UserRepositoryImpl::new(db);

        let user = User::new("".to_string(), username, password).unwrap();

        let create_user = repo.create(user).await;

        match create_user {
            Ok(user) => {
                let user = CreateUser {
                    id: user.0.id,
                    username: user.1.username,
                    password: user.1.password,
                };
                Ok(user)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
