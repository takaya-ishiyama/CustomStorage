use domain::{
    infrastructure::interface::repository::{
        repository_interface::Repositories, user_repository_interface::UserRepository,
    },
    models::user::User,
};

pub trait GetUserUseCase<R: Repositories> {
    fn get_user(&self, id: String) -> Result<User, String>;
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GetUserInteractor<'r, R: Repositories> {
    user_repo: &'r R::UserRepo,
}

impl<'r, R: Repositories> GetUserInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            user_repo: repositories.user_repo(),
        }
    }

    pub async fn get_user(&self, id: String) -> Result<User, String> {
        let user = self.user_repo;
        let user = user.find_by_id(id).await;
        Ok(user)
    }
}
