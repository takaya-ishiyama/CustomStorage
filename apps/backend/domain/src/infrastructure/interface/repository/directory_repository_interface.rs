use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

use crate::{
    infrastructure::dto::directories::create_input_dto::CreateInputDto,
    value_object::directory::Directory,
};

#[automock]
#[async_trait]
pub trait DirectoriesRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Directory>, String>;
    async fn create<'a>(&self, dto: CreateInputDto<'a>) -> Result<Directory, String>;
}
