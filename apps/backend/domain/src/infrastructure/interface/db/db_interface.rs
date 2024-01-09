use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait DbTrait {
    async fn new() -> Self;
}

pub async fn new_db<T: DbTrait>() -> T {
    T::new().await
}
