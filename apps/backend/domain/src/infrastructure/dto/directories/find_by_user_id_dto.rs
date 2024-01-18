use uuid::Uuid;

pub struct FindByUserIdDto {
    user_id: Uuid,
}

impl FindByUserIdDto {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: Uuid::parse_str(user_id).unwrap(),
        }
    }
    pub fn get_user_id(&self) -> Uuid {
        self.user_id
    }
}
