use async_graphql::*;
use domain::models::user::User;

#[derive(SimpleObject)]
struct Schema  {
    user_schema: User,
};
