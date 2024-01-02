use domain::{
    infrastructure::interface::repository::{
        repository_interface::Repositories, user_repository_interface::UserRepository,
    },
    models::{interface::user_interface::UserTrait, user::User},
    value_object::token::Session,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UserInteractor<'r, R: Repositories> {
    user_repo: &'r R::UserRepo,
}

impl<'r, R: Repositories> UserInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            user_repo: repositories.user_repo(),
        }
    }

    pub async fn get_user(&self, id: &str) -> Result<User, String> {
        let user = self.user_repo.find_by_id(id).await;
        Ok(user)
    }

    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(User, Session), String> {
        let input_user = User::new("", username, password).unwrap();
        let created_user = self.user_repo.create(input_user).await.unwrap();
        Ok(created_user)
    }
}
