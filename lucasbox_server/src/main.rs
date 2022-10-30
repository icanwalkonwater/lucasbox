use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use tokio::sync::Mutex;

use lucasbox_server::schema_graphql::{Mutation, Query, Schema, Subscription};

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8080/graphql")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:uwu@localhost:5432/lucasfilm";
    let connection = Arc::new(Mutex::new(
        AsyncPgConnection::establish(database_url).await.unwrap(),
    ));

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(connection)
    .finish();

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let addr = "[::1]:8080".parse().unwrap();

    println!("Listening at {}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
