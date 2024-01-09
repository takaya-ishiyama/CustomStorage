#[derive(Debug, Clone)]
pub struct UserValueObject {
    pub username: String,
    pub password: String,
}

impl UserValueObject {
    pub fn new(username: &str, password: &str) -> Result<Self, &'static str> {
        // バリデーション
        if username.is_empty() {
            return Err("Name cannot be empty");
        }

        return Ok(UserValueObject {
            username: username.to_string(),
            password: password.to_string(),
        });
    }
}

// テストモジュール
#[cfg(test)]
mod tests {
    // テスト対象の関数をインポート
    use super::UserValueObject;

    // テスト関数
    #[test]
    fn test_user_value_object_true() {
        let name = "test".to_string();
        let password = "test".to_string();
        let user_value_object = UserValueObject::new(&name, &password).unwrap();
        assert_eq!(user_value_object.username, "test");
        assert_eq!(user_value_object.password, "test");
    }

    #[test]
    fn test_user_value_object_null_name() {
        let name = "".to_string();
        let password = "test".to_string();
        let user_value_object = UserValueObject::new(&name, &password);
        assert!(user_value_object.is_err());
    }
}
