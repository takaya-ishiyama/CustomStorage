use sqlx::{pool, Pool, Postgres};

pub async fn setup_database() -> Pool<Postgres> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = pool::Pool::<Postgres>::connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!("src/db/migrations")
        .run(&pool)
        .await
        .unwrap();

    pool
}
