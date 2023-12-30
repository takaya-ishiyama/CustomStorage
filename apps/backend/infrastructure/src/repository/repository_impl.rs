use std::sync::Arc;

use domain::infrastructure::interface::repository::{
    repository_interface::Repositories, user_repository_interface::UserRepository,
};
use sqlx::{pool::PoolConnection, Pool, Postgres};

use super::user_repository::UserRepositoryImpl;

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;

    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            user_repo: UserRepositoryImpl::new(db),
        }
    }

    fn user_repo(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
