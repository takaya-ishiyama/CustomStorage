use crate::{db::persistence::postgres::Db, repository::repository_impl::RepositoryImpls};
use async_graphql::*;
use domain::infrastructure::interface::repository::repository_interface::Repositories;
use usecase::{session::usecase::SessionInteractor, user::usecase::UserInteractor};

use super::schema::auth::{SessionSchema, UserSchema};

#[derive(SimpleObject)]
struct GetUser {
    #[graphql(flatten)]
    user: UserSchema,
}

#[derive(SimpleObject)]
struct GetNewToken {
    #[graphql(flatten)]
    session: SessionSchema,
}

#[derive(SimpleObject)]
struct Login {
    #[graphql(flatten)]
    user: UserSchema,
    session: SessionSchema,
}

pub struct Query;

pub struct Token(pub String);

#[Object]
impl Query {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
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
            user: UserSchema::new(login.0 .0.id, login.0 .1.username),
            session: SessionSchema::new(
                None,
                login.1.user_id,
                Some(login.1.access_token),
                Some(login.1.refresh_token),
            ),
        })
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
            session: SessionSchema::new(
                None,
                session.user_id,
                Some(session.access_token),
                Some(session.refresh_token),
            ),
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
            user: UserSchema::new(_user.0.id, _user.1.username),
        };
        Ok(user)
    }
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
            user: UserSchema::new(user.0.id, user.1.username),
        };
        Ok(user)
    }
}
