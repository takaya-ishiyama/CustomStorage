use async_graphql::*;

#[derive(SimpleObject, sqlx::FromRow)]
struct Ping {
    status: String,
    code: i32,
}

pub struct Query;

#[Object]
impl Query {
    async fn ping(&self) -> Ping {
        Ping {
            status: "ok".to_string(),
            code: 200,
        }
    }

    async fn borrow_from_context_data<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx String> {
        ctx.data::<String>()
    }
}
