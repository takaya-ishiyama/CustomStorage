use crate::{db::persistence::postgres::Db, repository::repository_impl::RepositoryImpls};
use async_graphql::*;
use domain::infrastructure::interface::repository::repository_interface::Repositories;
use usecase::{session::usecase::SessionInteractor, user::usecase::UserInteractor};

#[derive(SimpleObject)]
struct GetUser {
    id: String,
    username: String,
    password: String,
}

#[derive(SimpleObject)]
struct GetNewToken {
    access_token: String,
    refresh_token: String,
}

pub struct Query;

pub struct Token(pub String);

#[Object]
impl Query {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }

    async fn get_new_token<'ctx>(&self, ctx: &Context<'ctx>) -> Result<GetNewToken, String> {
        let token = ctx.data_opt::<Token>().map(|token| token.0.as_str());
        if token.is_none() {
            return Err("token is none".to_string());
        }

        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);

        let session_usecase = SessionInteractor::new(&repo);

        let session = session_usecase.update_token(token.unwrap()).await.unwrap();

        Ok(GetNewToken {
            access_token: session.access_token,
            refresh_token: session.refresh_token,
        })
    }

    async fn login_with_token<'a>(&self, ctx: &'a Context<'_>) -> Result<GetUser, String> {
        let token = ctx.data_opt::<Token>().map(|token| token.0.as_str());
        if token.is_none() {
            return Err("token is none".to_string());
        }
        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);
        let user_usecase = UserInteractor::new(&repo);
        let _user = user_usecase.login_with_token(token.unwrap()).await.unwrap();
        let user = GetUser {
            id: _user.0.id,
            username: _user.1.username,
            password: _user.1.password,
        };
        Ok(user)
    }
    // async fn get_new_access_token<'a>(&self, ctx: &'a Context<'_>) -> Result<String, String> {
    //     let token = ctx.data_opt::<Token>().map(|token| token.0.as_str());
    //     if token.is_none() {
    //         return Err("token is none".to_string());
    //     }
    //     let db = ctx.data::<Db>().unwrap().0.clone();
    //     let repo = RepositoryImpls::new(db);
    //     let session_usecase = SessionInteractor::new(&repo);
    //     let session = session_usecase
    //         .get_access_token(token.unwrap())
    //         .await
    //         .unwrap();
    //     Ok(session.access_token)
    // }
    async fn get_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "Id of object")] id: String,
    ) -> Result<GetUser> {
        let db = ctx.data::<Db>().unwrap().0.clone();
        let repo = RepositoryImpls::new(db);
        let user_usecase = UserInteractor::new(&repo);
        let user = user_usecase.get_user(&id).await.unwrap();
        let user = GetUser {
            id: user.0.id,
            username: user.1.username,
            password: user.1.password,
        };
        Ok(user)
    }
}
