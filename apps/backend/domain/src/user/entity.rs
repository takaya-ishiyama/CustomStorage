use uuid::Uuid;

pub struct UserEntity {
    id: UserId,
}

struct UserId(Uuid);

impl UserEntity {
    pub fn new(id: Uuid) -> Self {
        UserEntity { id: UserId(id) }
    }
}
