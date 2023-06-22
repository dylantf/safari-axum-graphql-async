use std::sync::Arc;

use async_graphql::{dataloader::DataLoader, http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};

use crate::app_state::AppState;

mod schema;

pub async fn graphql_handler(
    schema: Extension<schema::SafariSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    let graphiql = GraphiQLSource::build().endpoint("/api/graphql").finish();
    response::Html(graphiql)
}

pub fn build_graphql_schema(app_state: Arc<AppState>) -> schema::SafariSchema {
    Schema::build(
        schema::Query::default(),
        schema::MutationRoot,
        EmptySubscription,
    )
    .data(DataLoader::new(
        schema::company::BatchCompanyById::new(Arc::clone(&app_state)),
        tokio::spawn,
    ))
    .data(DataLoader::new(
        schema::user::BatchUsersByCompanyId::new(Arc::clone(&app_state)),
        tokio::spawn,
    ))
    .data(Arc::clone(&app_state))
    .finish()
}
