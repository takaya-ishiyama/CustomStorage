use sqlx::PgPool;

pub async fn setup_database(pool: &PgPool) {
    sqlx::migrate!("src/db/migrations").run(pool).await.unwrap();
}
