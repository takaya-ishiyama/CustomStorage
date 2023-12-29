mod auth;
mod db;
mod graphql;
mod repository;

use std::net::SocketAddr;

use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql::{http::playground_source, EmptySubscription, Request, Response, Schema};
use graphql::mutation::Mutation;
use graphql::query::{Query, Token};

use axum::extract::{Json, State};
use axum::response::IntoResponse;
use axum::{extract::Extension, response::Html, routing::get, routing::post, Router};

use hyper::HeaderMap;
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

        let schema = Schema::build(Query, Mutation, EmptySubscription)
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

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| Token(s.to_string())).ok())
}

async fn graphql_handler(
    schema: Extension<Schema<Query, Mutation, EmptySubscription>>,
    headers: HeaderMap,
    req: Json<Request>,
) -> Json<Response> {
    let mut req = req.0;
    if let Some(token) = get_token_from_headers(&headers) {
        req = req.data(token);
    }
    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_graphql_handler_with_token() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "foo".parse().unwrap());

        let processed_handler = graphql_handler(
            Extension(Schema::build(Query, Mutation, EmptySubscription).finish()),
            headers,
            Json(Request::new("".to_string())),
        );

        assert_eq!(1, 1)
    }
}
