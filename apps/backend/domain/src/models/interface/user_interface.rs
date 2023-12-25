pub trait UserTrait {
    fn new(id: String, username: String, password: String) -> Result<Self, String>
    where
        Self: Sized;
}

pub fn new_user<T: UserTrait>(id: String, username: String, password: String) -> T {
    T::new(id, username, password).unwrap()
}
