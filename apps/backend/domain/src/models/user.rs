mod entity;
mod value_object;

use entity::UserEntity;
use value_object::UserValueObject;

#[derive(Debug, Clone)]
pub struct User(pub UserEntity, pub UserValueObject);

impl User {
    pub fn new(name: String, password: String) -> Result<Self, String> {
        match UserValueObject::new(name, password) {
            Ok(value_object) => Ok(User(UserEntity::new(), value_object)),
            Err(e) => panic!("入力値に誤りがある: {}", e),
        }
    }

    pub fn get_user(id: String, name: String, password: String) -> Result<Self, String> {
        Ok(User(
            UserEntity { id },
            UserValueObject::new(name, password).unwrap(),
        ))
    }
}

// // テストモジュール
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
