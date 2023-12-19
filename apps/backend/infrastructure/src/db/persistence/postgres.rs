use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<Postgres>>);

impl Db {
    pub async fn new() -> Db {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });

        Db(Arc::new(pool))
    }
}
