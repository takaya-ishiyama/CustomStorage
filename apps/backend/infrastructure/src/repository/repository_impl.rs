use std::sync::Arc;

use domain::infrastructure::interface::repository::{
    repository_interface::Repositories, session_repository_interface::SessionRepository,
    user_repository_interface::UserRepository,
};
use sqlx::{Pool, Postgres};

use super::{session_repository::SessionRepositoryImpl, user_repository::UserRepositoryImpl};

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
    token_repo: SessionRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;
    type SessionRepo = SessionRepositoryImpl;

    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            // FIXME: 参照を渡すようにする。できればコネクションを渡すようにする。
            // arcを渡すとcloneされるたびにコネクションが増えていく。コネクションが増えるだけなので問題ない可能性もある
            user_repo: UserRepositoryImpl::new(db.clone()),
            token_repo: SessionRepositoryImpl::new(db.clone()),
        }
    }

    fn user_repo(&self) -> &Self::UserRepo {
        &self.user_repo
    }

    fn session_repo(&self) -> &Self::SessionRepo {
        &self.token_repo
    }
}
