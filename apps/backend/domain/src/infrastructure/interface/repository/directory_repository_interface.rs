use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

use crate::value_object::directory::Directory;

#[automock]
#[async_trait]
pub trait DirectoriesRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Directory>, Error>;
    async fn create(&self, user_id: &str, parent_id: &str, name: &str) -> Result<Directory, Error>;
}
