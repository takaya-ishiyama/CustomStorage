use uuid::Uuid;

// #[derive(Debug, Clone)]
// pub struct UserEntity {
//     id: Uuid,
// }

// impl UserEntity {
//     pub fn new() -> Self {
//         let id = Uuid::new_v4();
//         UserEntity { id }
//     }
// }

// // テストモジュール
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
