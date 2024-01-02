use chrono::Local;
use domain::{
    infrastructure::interface::repository::{
        repository_interface::Repositories, user_repository_interface::UserRepository,
    },
    models::{interface::user_interface::UserTrait, user::User},
    value_object::token::{Session, SessionInterface},
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

    pub async fn login(&self, username: &str, password: &str) -> Result<(User, Session), String> {
        let find_user = self
            .user_repo
            .find_by_username_and_password(username, password)
            .await
            .unwrap();

        let user = User::new(
            &find_user.0.id,
            &find_user.1.username,
            &find_user.1.password,
        )
        .unwrap();

        let session = Session::new(
            &user.0.id,
            "",
            "",
            &Local::now().naive_local(),
            &Local::now().naive_local(),
        )
        .create();

        Ok((user, session))
    }
}
