mod auth;
mod db;
mod graphql;
mod repository;

use std::net::SocketAddr;

use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{
    http::playground_source, EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use graphql::query::Query;

use axum::extract::Json;
use axum::response::IntoResponse;
use axum::{extract::Extension, response::Html, routing::get, routing::post, Router};

use tower_http::cors::{Any, CorsLayer};

use db::persistence::postgres::Db;
use domain::infrastructure::interface::db::db_interface::new_db;

#[tokio::main]
async fn main() {
    let server = async {
        // FIXME: ANYなおす
        let cors = CorsLayer::new()
            // .allow_headers(vec![ContentType::json()])
            .allow_headers(Any)
            .allow_methods(Any)
            .allow_origin(Any);

        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(new_db::<Db>().await)
            .finish();

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .route("/graphql", post(graphql_handler))
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

pub type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<MySchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}
