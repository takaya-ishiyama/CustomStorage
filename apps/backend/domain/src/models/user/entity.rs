use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: String,
}

impl UserEntity {
    pub fn new() -> Self {
        let id = Uuid::new_v4().to_string();
        UserEntity { id }
    }

    pub fn get_user_id(&self) -> Result<String, String> {
        Ok(self.id.clone())
    }
}

// テストモジュール
// #[cfg(test)]
// mod tests {
//     // テスト対象の関数をインポート
//     use super::UserEntity;
//     use uuid::Uuid;

//     // テスト関数
//     #[test]
//     fn test_user_entity_true() {
//         let user_entity = UserEntity::new();
//         assert_ne!(user_entity.id, Uuid::nil());
//     }
// }
