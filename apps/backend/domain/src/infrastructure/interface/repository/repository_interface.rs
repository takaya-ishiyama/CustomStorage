use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::{
    token_repository_interface::TokenRepository, user_repository_interface::UserRepository,
};

#[async_trait]
pub trait Repositories {
    type UserRepo: UserRepository;
    type TokenRepo: TokenRepository;

    fn new(db: Arc<Pool<Postgres>>) -> Self;
    fn user_repo(&self) -> &Self::UserRepo;
    fn token_repo(&self) -> &Self::TokenRepo;
}
