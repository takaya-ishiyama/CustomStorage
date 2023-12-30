use domain::infrastructure::interface::repository::repository_interface::Repositories;

use super::user_repository::UserRepositoryImpl;

#[derive(Clone, Debug)]
pub(crate) struct RepositoryImpls {
    pub user_repo: UserRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type UserRepo = UserRepositoryImpl;

    fn user_repo(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}
