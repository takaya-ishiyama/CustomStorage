pub mod user_usecase;

use async_trait::async_trait;

use crate::user::get_user_interactor::GetUserUseCase;

// #[async_trait]
// pub trait Usecase<'r, R> {
//     type GetUser: GetUserUseCase;

//     fn get_user(&self) -> &Self::GetUser;
// }
