use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Extension, Router};
use graphql::{build_graphql_schema, graphiql_handler, graphql_handler};
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::AppState;

mod app_state;
mod config;
mod database;
mod entities;
mod graphql;

#[tokio::main]
async fn main() {
    let config = match crate::config::AppConfig::create().await {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("{}", e);
            panic!("Error reading config, exiting.");
        }
    };

    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .with_test_writer()
        .init();

    let db = database::connect_to_database(&config.database_url)
        .await
        .expect("Unable to connect to database!");

    let app_state = Arc::new(AppState::create(config, db));

    let schema = build_graphql_schema(app_state);

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
