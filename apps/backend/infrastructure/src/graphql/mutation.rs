use async_graphql::{Context, Object, SimpleObject};
use domain::infrastructure::interface::repository::repository_interface::Repositories;
use usecase::{session::usecase::SessionInteractor, user::usecase::UserInteractor};

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

#[derive(SimpleObject)]
struct Login {
    id: String,
    username: String,
    password: String,
    access_token: String,
    refresh_token: String,
}

pub struct Token(pub String);

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

    async fn get_new_access_token<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "refresh_token of object")] refresh_token: String,
    ) -> Result<String, String> {
        let token = ctx.data_opt::<Token>().map(|token| token.0.as_str());
        if token.is_none() {
            return Err("token is none".to_string());
        }

        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let session_usecase = SessionInteractor::new(&repo);

        let session = session_usecase
            .update_access_token(&refresh_token)
            .await
            .unwrap();

        Ok(session.access_token)
    }

    async fn login<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "name of object")] username: String,
        #[graphql(desc = "password of object")] password: String,
    ) -> Result<Login, String> {
        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let user_usecase = UserInteractor::new(&repo);

        let login = user_usecase.login(&username, &password).await.unwrap();

        Ok(Login {
            id: login.0 .0.id,
            username: login.0 .1.username,
            password: login.0 .1.password,
            access_token: login.1.access_token,
            refresh_token: login.1.refresh_token,
        })
    }
}
