pub mod entity;
pub mod value_object;

use entity::UserEntity;
use value_object::UserValueObject;

use super::interface::user_interface::UserTrait;

// value_objectディレクトリに移行する。データベース都合でドメインオブジェクトを作りたくないから

#[derive(Debug, Clone)]
pub struct User(pub UserEntity, pub UserValueObject);

impl UserTrait for User {
    fn new(id: &str, username: &str, password: &str) -> Result<Self, String> {
        Ok(User(
            UserEntity::new(id).unwrap(),
            UserValueObject::new(username, password).unwrap(),
        ))
    }
}

// #[cfg(test)]
// mod tests {
//     // テスト対象の関数をインポート
//     use super::User;

//     // テスト関数
//     #[test]
//     fn test_user_true() {
//         let name = "".to_string();
//         let password = "".to_string();
//         let result = std::panic::catch_unwind(|| User::new(name, password));
//         assert!(result.is_err());
//     }
// }
