use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::Object;
// use async_graphql::{EmptyMutation, EmptySubscription, Request, Response, Schema};
use async_graphql::{
    http::playground_source, EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::extract::{Json, Path};
use axum::response::IntoResponse;
use axum::{body::Body, extract::Extension, response::Html, routing::get, Router};
// use axum_login::axum::Extension;
// use bytes::Bytes;
// use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
mod graphql;
use graphql::query::QueryRoot;
// use tower_http::trace::TraceLayer;

// use http::{header, Method, Request, Response};
// use std::convert::Infallible;
// use tower::{Service, ServiceBuilder, ServiceExt};

// struct Query;

// #[Object]
// impl Query {
//     /// Returns the sum of a and b
//     async fn add(&self, a: i32, b: i32) -> i32 {
//         a + b
//     }
// }
#[tokio::main]
async fn main() {
    let server = async {
        let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .route("/plain_text", get(plain_text))
            .route("/json", get(json))
            .layer(cors)
            .layer(Extension(schema));

        let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
        // server を起動
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    };

    tokio::join!(server);
}

pub type UserSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<UserSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
async fn path(Path(user_id): Path<u32>) {}
// async fn query(Query(params): Query<HashMap<String, String>>) {}

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
