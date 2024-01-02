use chrono::Local;
use domain::{
    infrastructure::interface::repository::{
        repository_interface::Repositories, session_repository_interface::SessionRepository,
        user_repository_interface::UserRepository,
    },
    models::{interface::user_interface::UserTrait, user::User},
    value_object::token::{Session, SessionInterface},
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UserInteractor<'r, R: Repositories> {
    session_repo: &'r R::SessionRepo,
}

impl<'r, R: Repositories> UserInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            session_repo: repositories.session_repo(),
        }
    }

    pub async fn update_access_token(&self, refresh_token: &str) -> Result<Session, String> {
        let session = self
            .session_repo
            .find_by_refresh_token(refresh_token)
            .await
            .unwrap();

        let create_new_session = Session::new(
            &session.user_id,
            &session.access_token,
            &session.refresh_token,
            &session.expiration_timestamp,
        )
        .create();
        let input_session = Session::new(
            &session.user_id,
            &create_new_session.access_token,
            &session.refresh_token,
            &create_new_session.expiration_timestamp,
        );
        let session = self.session_repo.update(&input_session).await.unwrap();
        Ok(session)
    }
}
