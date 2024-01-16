use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::interface::repository::directory_repository_interface::DirectoriesRepository,
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

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Directory>, Error> {
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
        .bind(Uuid::parse_str(user_id).unwrap())
        .fetch_all(&mut *tx)
        .await;

        let dir_array = match rows {
            Err(e) => {
                tx.rollback().await.unwrap();
                return Err(e);
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

    async fn create(&self, user_id: &str, parent_id: &str, name: &str) -> Result<Directory, Error> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let rows = sqlx::query_as::<_, Create>(
            r#"
            INSERT INTO directories (user_id, name, parent_id)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(Uuid::parse_str(user_id).unwrap())
        .bind(name)
        .bind(Uuid::parse_str(parent_id).unwrap())
        .fetch_one(&mut *tx)
        .await;

        match rows {
            Err(e) => {
                tx.rollback().await.unwrap();
                return Err(e);
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
    async fn test_dir_repo(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = DirectoriesRepositoryImpl::new(db);

        assert_eq!(1, 1);

        Ok(())
    }
}
