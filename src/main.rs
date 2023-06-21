use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Extension, Router};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{self, EnvFilter};

use graphql::{build_graphql_schema, graphiql_handler, graphql_handler};

mod config;
mod database;
mod entities;
mod graphql;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_test_writer()
        .init();

    let config = match crate::config::Config::build().await {
        Ok(config) => Arc::new(config),
        Err(e) => {
            tracing::error!("{}", e);
            panic!("Error reading config, exiting.");
        }
    };

    let conn = database::connect_to_database(&config.database_url)
        .await
        .expect("Unable to connect to database!");

    let schema = build_graphql_schema(conn);

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/healthcheck", get(|| async { "alive!" }))
        .route("/api/graphql", get(graphiql_handler).post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
    println!("🚀 Server running at {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
