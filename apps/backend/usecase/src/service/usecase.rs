use domain::{
    infrastructure::{
        dto::directories::{
            create_input_dto::CreateInputDto, find_by_pearent_id_dto::FindByPearentIdDto,
            find_by_user_id_dto::FindByUserIdDto,
        },
        interface::repository::{
            directory_repository_interface::DirectoriesRepository,
            repository_interface::Repositories,
        },
    },
    value_object::{
        directory::Directory, interface::service_interface::ServiceInterface, service::Service,
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
                let service = ServiceInterface::new(Some(directories), None);
                Ok(service)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_own_service(
        &self,
        user_id: &str,
        pearent_id: &Option<String>,
    ) -> Result<Service, String> {
        match pearent_id {
            Some(pearent_id) => {
                let directory_dto = FindByPearentIdDto::new(pearent_id);

                let directories = self
                    .directories_repo
                    .find_by_pearent_id(&directory_dto)
                    .await;

                match directories {
                    Ok(directories) => {
                        let service = ServiceInterface::new(Some(directories), None);
                        Ok(service)
                    }
                    Err(e) => Err(e),
                }
            }
            None => {
                let directory_dto = FindByUserIdDto::new(user_id);
                let directories = self.directories_repo.find_by_user_id(&directory_dto).await;

                match directories {
                    Ok(directories) => {
                        let service = ServiceInterface::new(Some(directories), None);
                        Ok(service)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub async fn create_directory(
        &self,
        user_id: &str,
        name: &str,
        parent_id: Option<&str>,
    ) -> Result<Directory, String> {
        let directory_dto = CreateInputDto::new(user_id, &parent_id, name);
        let directory = self.directories_repo.create(&directory_dto).await;

        match directory {
            Ok(directory) => Ok(directory),
            Err(e) => Err(e),
        }
    }
}
