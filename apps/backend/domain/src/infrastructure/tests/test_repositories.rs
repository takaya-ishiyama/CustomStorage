use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::infrastructure::interface::repository::{
    repository_interface::Repositories, session_repository_interface::MockSessionRepository,
    user_repository_interface::MockUserRepository,
};

// pub struct TestRepositories {
//     user_repo: MockUserRepository,
//     session_repo: MockSessionRepository,
// }

// impl Repositories for TestRepositories {
//     type UserRepo = MockUserRepository;
//     type SessionRepo = MockSessionRepository;

//     fn new(_db: Arc<Pool<Postgres>>) -> Self {
//         unimplemented!()
//     }

//     fn user_repo(&self) -> &Self::UserRepo {
//         &self.user_repo
//     }

//     fn session_repo(&self) -> &Self::SessionRepo {
//         &self.session_repo
//     }
// }
