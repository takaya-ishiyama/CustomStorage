use async_graphql::{Context, Object, SimpleObject};
use domain::infrastructure::interface::repository::repository_interface::Repositories;
use usecase::user::usecase::UserInteractor;

use crate::{db::persistence::postgres::Db, repository::repository_impl::RepositoryImpls};

use super::schema::auth::{SessionSchema, UserSchema};

pub struct Mutation;

#[derive(SimpleObject)]
struct CreateUser {
    #[graphql(flatten)]
    user: UserSchema,
    session: SessionSchema,
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
            user: UserSchema::new(create_user.0 .0.id, create_user.0 .1.username),
            session: SessionSchema::new(
                None,
                create_user.1.user_id,
                Some(create_user.1.access_token),
                Some(create_user.1.refresh_token),
            ),
        })
    }
}
