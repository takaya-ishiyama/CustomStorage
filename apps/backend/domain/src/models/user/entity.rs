#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: String,
}

impl UserEntity {
    pub fn new(id: &str) -> Result<Self, &'static str> {
        // if id.is_empty() {
        //     return Err("ID cannot be empty");
        // }
        return Ok(UserEntity { id: id.to_string() });
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
