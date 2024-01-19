use std::sync::Arc;

use async_trait::async_trait;
use domain::{
    infrastructure::{
        dto::directories::{
            create_input_dto::CreateInputDto, find_by_pearent_id_dto::FindByPearentIdDto,
            find_by_user_id_dto::FindByUserIdDto,
        },
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
struct Find {
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

    async fn find_by_user_id(&self, dto: &FindByUserIdDto) -> Result<Vec<Directory>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();
        let mut items = Vec::new();
        let rows = sqlx::query_as::<_, Find>(
            r#"
            SELECT
                *
            FROM
                directories
            WHERE
                user_id = $1
            "#,
        )
        .bind(dto.get_user_id())
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

    async fn find_by_pearent_id(&self, dto: &FindByPearentIdDto) -> Result<Vec<Directory>, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();
        let mut items = Vec::new();
        let rows = sqlx::query_as::<_, Find>(
            r#"
            SELECT
                *
            FROM
                directories
            WHERE
                parent_id = $1
            "#,
        )
        .bind(dto.get_pearent_id())
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

    async fn create<'a>(&self, dto: &CreateInputDto<'a>) -> Result<Directory, String> {
        let mut pool = self.db.acquire().await.unwrap();
        let conn = pool.acquire().await.unwrap();
        let mut tx = conn.begin().await.unwrap();

        let rows = sqlx::query_as::<_, Create>(
            "
            INSERT INTO directories (user_id, name, parent_id)
            VALUES ($1, $2, $3)
            RETURNING *
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

        let dir = repo.create(&dto).await.unwrap();

        assert_eq!(dir.get_name(), "test_dir");

        Ok(())
    }

    #[sqlx::test]
    async fn test_dir_repo_create_with_parent_id(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = DirectoriesRepositoryImpl::new(db);

        let user = get_test_user();

        let dto = CreateInputDto::new(&user[0].0.id, &None, "test_dir");
        let pearent_dir = repo.create(&dto).await.unwrap();

        let dto = CreateInputDto::new(&user[0].0.id, &Some(pearent_dir.get_id()), "test_dir_2");

        let dir = repo.create(&dto).await.unwrap();

        assert_eq!(dir.get_parent_id().unwrap(), pearent_dir.get_id());

        Ok(())
    }

    #[sqlx::test]
    async fn test_dir_repo_find_by_user_id(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = DirectoriesRepositoryImpl::new(db);

        let user = get_test_user();

        let dto = CreateInputDto::new(&user[0].0.id, &None, "test_dir");
        let dir = repo.create(&dto).await.unwrap();

        let dto = FindByUserIdDto::new(&user[0].0.id);
        let dirs = repo.find_by_user_id(&dto).await.unwrap();

        assert_eq!(dirs[0].get_id(), dir.get_id());

        Ok(())
    }

    #[sqlx::test]
    async fn test_dir_repo_find_by_user_id_any_dir(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = DirectoriesRepositoryImpl::new(db);

        let user = get_test_user();

        let dto = CreateInputDto::new(&user[0].0.id, &None, "test_dir");
        let dir = repo.create(&dto).await.unwrap();

        let dto = CreateInputDto::new(&user[0].0.id, &Some(dir.get_id()), "test_dir_2");
        repo.create(&dto).await.unwrap();

        let dto = FindByUserIdDto::new(&user[0].0.id);
        let dirs = repo.find_by_user_id(&dto).await.unwrap();

        assert_eq!(dirs.len(), 2);

        Ok(())
    }

    #[sqlx::test]
    async fn test_dir_repo_find_by_pearent_id(pool: PgPool) -> sqlx::Result<()> {
        setup_database(&pool).await;
        let db = Arc::new(pool);
        let repo = DirectoriesRepositoryImpl::new(db);

        let user = get_test_user();

        let dto = CreateInputDto::new(&user[0].0.id, &None, "test_dir");
        let pearent_dir = repo.create(&dto).await.unwrap();

        let dto = CreateInputDto::new(&user[0].0.id, &Some(pearent_dir.get_id()), "test_dir_2");
        let children_dir = repo.create(&dto).await.unwrap();

        let dto = FindByPearentIdDto::new(pearent_dir.get_id());
        let dirs = repo.find_by_pearent_id(&dto).await.unwrap();

        assert_eq!(dirs[0].get_id(), children_dir.get_id());

        Ok(())
    }
}
