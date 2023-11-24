use uuid::Uuid;

pub struct UserEntity {
    id: UserId,
}

struct UserId(Uuid);

impl UserEntity {
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        UserEntity { id: UserId(id) }
    }
}

// テストモジュール
#[cfg(test)]
mod tests {
    // テスト対象の関数をインポート
    use super::UserEntity;
    use uuid::Uuid;

    // テスト関数
    #[test]
    fn test_user_entity_true() {
        let user_entity = UserEntity::new();
        assert_ne!(user_entity.id.0, Uuid::nil());
    }
}
