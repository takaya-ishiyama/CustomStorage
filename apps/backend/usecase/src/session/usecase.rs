use domain::{
    infrastructure::interface::repository::{
        repository_interface::Repositories, session_repository_interface::SessionRepository,
    },
    value_object::token::{Session, SessionInterface},
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SessionInteractor<'r, R: Repositories> {
    session_repo: &'r R::SessionRepo,
}

impl<'r, R: Repositories> SessionInteractor<'r, R> {
    pub fn new(repositories: &'r R) -> Self {
        Self {
            session_repo: repositories.session_repo(),
        }
    }

    pub async fn update_token(&self, refresh_token: &str) -> Result<Session, String> {
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
            &session.expiration_timestamp_for_refresh,
        )
        .create();
        let session = self.session_repo.update(&create_new_session).await.unwrap();
        Ok(session)
    }

    pub async fn get_access_token(&self, refresh_token: &str) -> Result<Session, String> {
        let session = self
            .session_repo
            .find_by_refresh_token(refresh_token)
            .await
            .unwrap();
        Ok(session)
    }
}

// #[cfg(test)]
// mod tests {

//     use domain::infrastructure::interface::{
//         db::db_interface::MockDbTrait,
//         repository::{
//             session_repository_interface::MockSessionRepository,
//             user_repository_interface::{MockUserRepository, UserRepository},
//         },
//     };

//     use super::*;

//     #[test]
//     fn test_session_interactor_update_access_token() {
//         let mut session_repo = MockSessionRepository::new();
//         assert_eq!(1, 1)
//     }
// }
