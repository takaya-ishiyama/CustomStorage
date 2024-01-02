use domain::models::{interface::user_interface::UserTrait, user::User};

pub fn get_test_user() -> Vec<User> {
    let user = User::new("17b5ac0c-1429-469a-8522-053f7bf0f80d", "test", "password").unwrap();
    Vec::from([user])
}
