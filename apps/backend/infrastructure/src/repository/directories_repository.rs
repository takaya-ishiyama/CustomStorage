use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::{
        dto::directories::create_input_dto::CreateInputDto,
        interface::repository::directory_repository_interface::DirectoriesRepository,
    },
    value_object::directory::Directory,
};
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime, Acquire, Error, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DirectoriesRepositoryImpl {
    db: Arc<Pool<Postgres>>,
}

#[derive(FromRow)]
struct FindByUserId {
    id: Uuid,
    user_id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
    created_at: NaiveDateTime,
}

#[derive(FromRow)]
struct Create {
    id: Uuid,
    user_id: Uuid,
    name: String,
    parent_id: Option<Uuid>,
    created_at: NaiveDateTime,
}

#[async_trait]
impl DirectoriesRepository for DirectoriesRepositoryImpl {
    fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Directory>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();
        let mut items = Vec::new();
        let rows = sqlx::query_as::<_, FindByUserId>(
            r#"
            SELECT
                *
            FROM
                directories
            WHERE
                user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&mut *tx)
        .await;

        let dir_array = match rows {
            Err(e) => {
                tx.rollback().await.unwrap();
                return Err(e.to_string());
            }
            Ok(rows) => {
                tx.commit().await.unwrap();
                rows
            }
        };

        for dir in dir_array.iter() {
            items.push(Directory::new(
                dir.id.to_string(),
                dir.user_id.to_string(),
                dir.name.to_string(),
                dir.parent_id.map(|id| id.to_string()),
                dir.created_at,
            ));
        }

        Ok(items)
    }

    async fn create<'a>(&self, dto: CreateInputDto<'a>) -> Result<Directory, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let rows = sqlx::query_as::<_, Create>(
            "
            INSERT INTO directories (user_id, name, parent_id)
            VALUES ($1, $2, $3)
            ",
        )
        .bind(dto.get_user_id())
        .bind(dto.get_name())
        .bind(dto.get_parent_id())
        .fetch_one(&mut *tx)
        .await;

        match rows {
            Err(e) => {
                tx.rollback().await.unwrap();
                return Err(e.to_string());
            }
            Ok(dir) => {
                tx.commit().await.unwrap();
                Ok(Directory::new(
                    dir.id.to_string(),
                    dir.user_id.to_string(),
                    dir.name.to_string(),
                    dir.parent_id.map(|id| id.to_string()),
                    dir.created_at,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use sqlx::PgPool;

    use crate::test::{setup_testdb::setup_database, test_data::get_test_user};

    use super::*;

    #[sqlx::test]
    async fn test_dir_repo_create_parent_id_null(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = DirectoriesRepositoryImpl::new(db);

        let user = get_test_user();

        let dto = CreateInputDto::new(&user[0].0.id, &None, "test_dir");

        let dir = repo.create(dto).await.unwrap();

        assert_eq!(dir.get_name(), "test_dir");

        Ok(())
    }
}
