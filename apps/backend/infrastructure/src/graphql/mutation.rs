use async_graphql::{Context, Object, SimpleObject};
use domain::infrastructure::interface::repository::repository_interface::Repositories;
use usecase::user::usecase::UserInteractor;

use crate::{db::persistence::postgres::Db, repository::repository_impl::RepositoryImpls};

pub struct Mutation;

#[derive(SimpleObject)]
struct CreateUser {
    id: String,
    username: String,
    password: String,
    access_token: String,
    refresh_token: String,
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
        let repo = RepositoryImpls::new(db);

        let user_usecase = UserInteractor::new(&repo);

        let create_user = user_usecase
            .create_user(&username, &password)
            .await
            .unwrap();

        Ok(CreateUser {
            id: create_user.0 .0.id,
            username: create_user.0 .1.username,
            password: create_user.0 .1.password,
            access_token: create_user.1.access_token,
            refresh_token: create_user.1.refresh_token,
        })
    }
}
