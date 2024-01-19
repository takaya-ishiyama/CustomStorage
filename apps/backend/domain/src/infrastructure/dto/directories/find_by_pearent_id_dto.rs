use uuid::Uuid;

pub struct FindByPearentIdDto {
    pearent_id: Uuid,
}

impl FindByPearentIdDto {
    pub fn new(pearent_id: &str) -> Self {
        Self {
            pearent_id: Uuid::parse_str(pearent_id).unwrap(),
        }
    }
    pub fn get_pearent_id(&self) -> Uuid {
        self.pearent_id
    }
}
