use async_graphql::{Context, Object, SimpleObject};
use domain::infrastructure::interface::repository::repository_interface::Repositories;
use usecase::{service::usecase::ServiceInteractor, user::usecase::UserInteractor};

use crate::{db::persistence::postgres::Db, repository::repository_impl::RepositoryImpls};

use super::schema::{
    auth::{SessionSchema, UserSchema},
    service::DirectorySchema,
};

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

    async fn create_directory<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "user_id")] user_id: String,
        #[graphql(desc = "name")] name: String,
        #[graphql(desc = "parent_id")] parent_id: Option<String>,
    ) -> Result<DirectorySchema, String> {
        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let user_usecase = ServiceInteractor::new(&repo);

        let directory = user_usecase
            .create_directory(&user_id, &name, parent_id.as_deref())
            .await;

        match directory {
            Ok(directory) => Ok(DirectorySchema::new(&directory)),
            Err(e) => Err(e),
        }
    }
}
