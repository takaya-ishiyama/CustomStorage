pub trait UserTrait {
    fn new(id: String, username: String, password: String) -> Result<Self, String>
    where
        Self: Sized;
}
