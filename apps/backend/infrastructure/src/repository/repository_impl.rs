use std::sync::Arc;

use domain::infrastructure::interface::repository::{
    directory_repository_interface::DirectoriesRepository,
    items_repository_interface::ItemsRepository, repository_interface::Repositories,
    session_repository_interface::SessionRepository, user_repository_interface::UserRepository,
};
use sqlx::{Pool, Postgres};

use super::{
    directories_repository::DirectoriesRepositoryImpl, irems_repository::ItemsRepositoryImpl,
    session_repository::SessionRepositoryImpl, user_repository::UserRepositoryImpl,
};

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    user_repo: UserRepositoryImpl,
    session_repo: SessionRepositoryImpl,
    directories_repo: DirectoriesRepositoryImpl,
    items_repo: ItemsRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;
    type SessionRepo = SessionRepositoryImpl;
    type DirectoriesRepo = DirectoriesRepositoryImpl;
    type ItemsRepo = ItemsRepositoryImpl;

    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            // FIXME: 参照を渡すようにする。できればコネクションを渡すようにする。
            // arcを渡すとcloneされるたびにコネクションが増えていく。コネクションが増えるだけなので問題ない可能性もある
            user_repo: UserRepositoryImpl::new(db.clone()),
            session_repo: SessionRepositoryImpl::new(db.clone()),
            directories_repo: DirectoriesRepositoryImpl::new(db.clone()),
            items_repo: ItemsRepositoryImpl::new(db.clone()),
        }
    }

    fn user_repo(&self) -> &Self::UserRepo {
        &self.user_repo
    }

    fn session_repo(&self) -> &Self::SessionRepo {
        &self.session_repo
    }

    fn directories_repo(&self) -> &Self::DirectoriesRepo {
        &self.directories_repo
    }

    fn items_repo(&self) -> &Self::ItemsRepo {
        &self.items_repo
    }
}
