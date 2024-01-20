use domain::{
    infrastructure::{
        dto::directories::find_by_user_id_dto::FindByUserIdDto,
        interface::repository::{
            directory_repository_interface::DirectoriesRepository,
            repository_interface::Repositories, session_repository_interface::SessionRepository,
        },
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

    pub async fn get_root_directory(&self, user_id: &str) -> Result<Service, String> {
        let directory_dto = FindByUserIdDto::new(user_id);
        let directories = self.directories_repo.find_by_user_id(&directory_dto).await;

        match directories {
            Ok(directories) => {
                let service = Service::new(Some(directories), None);
                Ok(service)
            }
            Err(e) => Err(e),
        }
    }
}
