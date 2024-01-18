use domain::{
    infrastructure::interface::repository::{
        repository_interface::Repositories, session_repository_interface::SessionRepository,
    },
    value_object::{
        service::Service,
        token::{Session, SessionInterface},
    },
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ServiceInteractor<'r, R: Repositories> {
    directories_repo: &'r R::DirectoriesRepo,
    items_repo: &'r R::ItemsRepo,
}

impl<'r, R: Repositories> ServiceInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            directories_repo: repositories.directories_repo(),
            items_repo: repositories.items_repo(),
        }
    }

    pub fn get_current_directory_servicies(
        &self,
        user_id: &str,
        current_dir: &str,
    ) -> Result<Service, String> {
    }
}
