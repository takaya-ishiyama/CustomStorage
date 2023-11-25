use async_graphql::*;
use domain::models::user::User;

struct Query;

#[Object]
impl Query {
    async fn user(&self, username: String) -> Result<Option<User>> {
        // Look up users from the database
    }
}
