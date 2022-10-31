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
use jwt_simple::algorithms::HS512Key;
use tokio::sync::Mutex;

use lucasbox_server::{
    schema_graphql::{Mutation, Query, Schema, Subscription},
    GlobalConfig,
};

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
    #[cfg(not(debug_assertions))]
    dotenvy::from_filename(".env.prod").ok();
    #[cfg(debug_assertions)]
    dotenvy::from_filename(".env.dev").ok();

    dotenvy::dotenv().ok();

    let config = GlobalConfig {
        database_url: std::env::var("DATABASE_URL").unwrap(),
        invite_code: std::env::var("INVITE_CODE").ok(),
        bind_address: std::env::var("BIND_ADDRESS")
            .unwrap_or_else(|_| "[::1]:8080".to_string())
            .parse()
            .unwrap(),
        // Regenerate the jwt key each time so old tokens are automatically made invalid
        jwt_key: HS512Key::generate(),
    };

    dbg!(&config);

    let connection = Arc::new(Mutex::new(
        AsyncPgConnection::establish(&config.database_url)
            .await
            .unwrap(),
    ));

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(connection)
    .data(config.clone())
    .finish();

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("Listening at {}", &config.bind_address);

    Server::bind(&config.bind_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
