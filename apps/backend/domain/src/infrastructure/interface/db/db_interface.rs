use async_trait::async_trait;

#[async_trait]
pub trait DbTrait {
    async fn new() -> Self;
}

pub async fn new_db<T: DbTrait>() -> T {
    T::new().await
}
