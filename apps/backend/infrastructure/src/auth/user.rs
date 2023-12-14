// use super::Db;
// // use domain::models::user::User as DomainUser;
// use password_auth::verify_password;
// use std::collections::HashMap;

// use async_trait::async_trait;
// use axum_login::{AuthUser, AuthnBackend, UserId};
// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;

// #[derive(Clone, Serialize, Deserialize, FromRow)]
// pub struct User {
//     id: String,
//     pub username: String,
//     password: String,
// }

// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("User")
//             .field("id", &self.id)
//             .field("username", &self.username)
//             .field("password", &"[redacted]")
//             .finish()
//     }
// }

// impl AuthUser for User {
//     type Id = String;

//     fn id(&self) -> Self::Id {
//         self.id
//     }

//     fn session_auth_hash(&self) -> &[u8] {
//         self.password.as_bytes()
//     }
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct Credentials {
//     pub username: String,
//     pub password: String,
//     pub next: Option<String>,
// }

// #[derive(Clone)]
// pub struct Backend {
//     db: Db,
// }

// impl Backend {
//     pub fn new(db: Db) -> Self {
//         Self { db }
//     }
// }

// #[async_trait]
// impl AuthnBackend for Backend {
//     type User = User;
//     type Credentials = Credentials;
//     type Error = sqlx::Error;

//     async fn authenticate(
//         &self,
//         creds: Self::Credentials,
//     ) -> Result<Option<Self::User>, Self::Error> {
//         let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
//             .bind(creds.username)
//             .fetch_optional(&self.db)
//             .await?;

//         Ok(user.filter(|user| {
//             verify_password(creds.password, &user.password)
//                 .ok()
//                 .is_some()
//         }))
//     }

//     async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
//         let user = sqlx::query_as("select * from users where id = ?")
//             .bind(user_id)
//             .fetch_optional(&self.db)
//             .await?;

//         Ok(user)
//     }
// }

// pub type AuthSession = axum_login::AuthSession<Backend>;
