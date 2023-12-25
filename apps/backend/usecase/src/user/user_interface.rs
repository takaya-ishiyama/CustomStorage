use async_trait::async_trait;

#[async_trait]
trait UserUsecase {
    async fn get_user(&self, id: String);
    async fn create_user(&self, name: String, password: String);
}

async fn user_usecase<T: UserUsecase>(args: T, id: String, name: String, password: String) {
    args.get_user(id).await;
    args.create_user(name, password).await;
}
