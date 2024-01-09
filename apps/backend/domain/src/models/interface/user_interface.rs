pub trait UserTrait {
    fn new(id: &str, username: &str, password: &str) -> Result<Self, String>
    where
        Self: Sized;
}
