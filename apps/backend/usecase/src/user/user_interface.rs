use async_trait::async_trait;

#[async_trait]
trait UserUsecase {
    async fn get_user(&self, id: String);
    async fn create_user(&self, name: String, password: String);
}
