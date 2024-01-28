use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::dto::items::find_by_directory_id_dto::FindByDirectoryIdDto,
    infrastructure::interface::repository::items_repository_interface::ItemsRepository,
    value_object::items::{Item, Items},
};
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime, Acquire, Pool, Postgres};
use uuid::Uuid;

#[derive(FromRow)]
struct FindByDirectoryId {
    id: Uuid,
    directories_id: Uuid,
    texts: String,
    title: String,
    created_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct ItemsRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[async_trait]
impl ItemsRepository for ItemsRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }
    async fn find_by_directory_id(&self, dto: &FindByDirectoryIdDto) -> Result<Vec<Item>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let rows = sqlx::query_as::<_, FindByDirectoryId>(
            "
            SELECT * FROM items
            WHERE directories_id = $1
            ",
        )
        .bind(dto.get_directories_id())
        .fetch_all(&mut *tx)
        .await;

        match rows {
            Ok(rows) => {
                let mut items = Vec::new();

                for row in rows {
                    items.push(Item::new(
                        row.id.to_string().as_str(),
                        row.directories_id.to_string().as_str(),
                        &row.texts,
                        &row.title,
                        row.created_at,
                    ))
                }
                Ok(items)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
