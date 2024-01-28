use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sqlx::{Pool, Postgres};

use crate::{
    infrastructure::dto::items::find_by_directory_id_dto::FindByDirectoryIdDto,
    value_object::items::Item,
};

#[automock]
#[async_trait]
pub trait ItemsRepository {
    fn new(db: Arc<Pool<Postgres>>) -> Self;
    async fn find_by_directory_id(&self, dto: &FindByDirectoryIdDto) -> Result<Vec<Item>, String>;
}
