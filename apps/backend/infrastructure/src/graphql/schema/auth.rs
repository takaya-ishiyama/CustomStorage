use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct UserSchema {
    id: String,
    username: String,
}

#[derive(SimpleObject)]
pub struct SessionSchema {
    id: Option<i32>,
    user_id: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl UserSchema {
    pub fn new(id: String, username: String) -> Self {
        Self { id, username }
    }
}

impl SessionSchema {
    pub fn new(
        id: Option<i32>,
        user_id: String,
        access_token: Option<String>,
        refresh_token: Option<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            access_token,
            refresh_token,
        }
    }
}
