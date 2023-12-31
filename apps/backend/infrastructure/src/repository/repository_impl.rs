use std::sync::Arc;

use domain::infrastructure::interface::repository::{
    repository_interface::Repositories, token_repository_interface::TokenRepository,
    user_repository_interface::UserRepository,
};
use sqlx::{Pool, Postgres};

use super::{token_repository::TokenRepositoryImpl, user_repository::UserRepositoryImpl};

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
    token_repo: TokenRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;
    type TokenRepo = TokenRepositoryImpl;

    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            // FIXME: 参照を渡すようにする。できればコネクションを渡すようにする。
            // arcを渡すとcloneされるたびにコネクションが増えていく。コネクションが増えるだけなので問題ない可能性もある
            user_repo: UserRepositoryImpl::new(db.clone()),
            token_repo: TokenRepositoryImpl::new(db.clone()),
        }
    }

    fn user_repo(&self) -> &Self::UserRepo {
        &self.user_repo
    }

    fn token_repo(&self) -> &Self::TokenRepo {
        &self.token_repo
    }
}
