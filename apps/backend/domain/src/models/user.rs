mod entity;
mod value_object;

use entity::UserEntity;
use value_object::UserValueObject;
pub struct User(UserEntity, UserValueObject);

impl User {
    pub fn new(name: String, password: String) -> Self {
        match UserValueObject::new(name, password) {
            Ok(value_object) => User(UserEntity::new(), value_object),
            Err(e) => panic!("入力値に誤りがある: {}", e),
        }
    }
}

// テストモジュール
#[cfg(test)]
mod tests {
    // テスト対象の関数をインポート
    use super::User;

    // テスト関数
    #[test]
    fn test_user_true() {
        let name = "".to_string();
        let password = "".to_string();
        let result = std::panic::catch_unwind(|| User::new(name, password));
        assert!(result.is_err());
    }
}
