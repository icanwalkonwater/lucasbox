use std::sync::Arc;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    headers::{authorization::Bearer, Authorization},
    response::{Html, IntoResponse},
    routing::get,
    Router, Server, TypedHeader,
};
use axum::http::Method;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use jwt_simple::algorithms::HS512Key;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use lucasbox_server::{
    auth::verify_jwt_token,
    schema_graphql::{Mutation, Query, Schema, Subscription},
    GlobalConfig,
};

async fn graphql_handler(
    schema: Extension<Schema>,
    global_config: Extension<GlobalConfig>,
    auth: Option<TypedHeader<Authorization<Bearer>>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let auth_info = auth
        .and_then(|token| verify_jwt_token(&global_config, token.token()));

    let req = req.into_inner();
    let request = if let Some(auth_info) = auth_info {
        req.data(auth_info)
    } else {
        req
    };

    schema.execute(request).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:8080/graphql")
            .credentials("include")
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

    env_logger::init();

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
    .extension(async_graphql::extensions::Logger)
    .finish();

    let cors = CorsLayer::new()
        .allow_methods([Method::OPTIONS, Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(Extension(config.clone()))
        .layer(Extension(schema));

    println!("Listening at {}", &config.bind_address);

    Server::bind(&config.bind_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
