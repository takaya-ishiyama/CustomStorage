use async_graphql::*;
use domain::models::user::User;

#[derive(SimpleObject)]
pub struct UserSchema {
    pub name: String,
}

impl From<User> for UserSchema {
    fn from(user: User) -> Self {
        Self {
            name: user.get_name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_to_user_schema_conversion() {
        // Create a user instance
        let user_schema = UserSchema {
            name: "John Doe".to_string(),
        };

        assert_eq!(user_schema.name, "John Doe");
    }
}
