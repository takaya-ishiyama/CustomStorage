use uuid::Uuid;

pub struct CreateInputDto<'a> {
    name: &'a str,
    parent_id: Option<Uuid>,
    user_id: Uuid,
}

impl<'a> CreateInputDto<'a> {
    pub fn new(user_id: &str, parent_id: &Option<&str>, name: &'a str) -> Self {
        Self {
            name,
            parent_id: parent_id
                .as_ref()
                .map(|parent_id| Uuid::parse_str(parent_id).unwrap()),
            user_id: Uuid::parse_str(user_id).unwrap(),
        }
    }
    pub fn get_name(&self) -> &'a str {
        self.name
    }
    pub fn get_parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }
    pub fn get_user_id(&self) -> Uuid {
        self.user_id
    }
}
