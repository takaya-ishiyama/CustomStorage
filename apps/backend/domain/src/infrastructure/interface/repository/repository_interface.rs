use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use super::{
    directory_repository_interface::DirectoriesRepository,
    items_repository_interface::ItemsRepository, session_repository_interface::SessionRepository,
    user_repository_interface::UserRepository,
};

#[async_trait]
pub trait Repositories {
    type UserRepo: UserRepository;
    type SessionRepo: SessionRepository;
    type DirectoriesRepo: DirectoriesRepository;
    type ItemsRepo: ItemsRepository;

    fn new(db: Arc<Pool<Postgres>>) -> Self;
    fn user_repo(&self) -> &Self::UserRepo;
    fn session_repo(&self) -> &Self::SessionRepo;
    fn directories_repo(&self) -> &Self::DirectoriesRepo;
    fn items_repo(&self) -> &Self::ItemsRepo;
}

pub trait TestRepositories {
    type UserRepo: UserRepository;
    type SessionRepo: SessionRepository;

    fn user_repo(&self) -> &Self::UserRepo;
    fn session_repo(&self) -> &Self::SessionRepo;
}
