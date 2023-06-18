use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

use crate::handlers::*;
use graphql::{build_graphql_schema, graphiql_handler, graphql_handler};

mod database;
mod entities;
mod graphql;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .with_test_writer()
        .init();

    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let db = database::connect_to_database(&connection_string)
        .await
        .expect("Unable to connect to database!");

    let app_state = AppState { db };

    let schema = build_graphql_schema(app_state.clone());

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/healthcheck", get(|| async { "alive!" }))
        .route("/api/graphql", get(graphiql_handler).post(graphql_handler))
        .route("/api/users", get(user_list_handler))
        .layer(Extension(schema))
        .with_state(app_state)
        .layer(cors);

    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
    println!("🚀 Server running at {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
