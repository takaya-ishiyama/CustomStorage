use axum::extract::{Json, Path, Query};
use axum::{body::Body, routing::get, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/plain_text", get(plain_text))
        .route("/json", get(json));
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}

async fn path(Path(user_id): Path<u32>) {}
async fn query(Query(params): Query<HashMap<String, String>>) {}

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
