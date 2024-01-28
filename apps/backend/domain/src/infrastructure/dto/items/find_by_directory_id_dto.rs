use uuid::Uuid;

pub struct FindByDirectoryIdDto {
    directories_id: Uuid,
}

impl FindByDirectoryIdDto {
    pub fn new(directories_id: &str) -> Self {
        Self {
            directories_id: Uuid::parse_str(directories_id).unwrap(),
        }
    }
    pub fn get_directories_id(&self) -> Uuid {
        self.directories_id
    }
}
