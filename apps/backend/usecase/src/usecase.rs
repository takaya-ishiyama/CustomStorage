use async_trait::async_trait;

#[async_trait]
pub trait Usecase {}

pub async fn usecase<T: Usecase>(args: T) {}
