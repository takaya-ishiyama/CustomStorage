mod entity;
mod value_object;

use entity::UserEntity;
use value_object::UserValueObject;
pub struct User(UserEntity, UserValueObject);

impl User {
    pub fn new(name: String, password: String) -> Result<Self, String> {
        match UserValueObject::new(name, password) {
            Ok(value_object) => Ok(User(UserEntity::new(), value_object)),
            Err(e) => panic!("入力値に誤りがある: {}", e),
        }
    }
    pub fn get_name(&self) -> String {
        self.1.name.clone()
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
