use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::HeaderMap,
    response::{self, IntoResponse},
    Extension,
};

use crate::app_state::AppState;
use schema::{company::BatchCompanyById, user::BatchUsersByCompanyId};

mod schema;

pub struct GqlContext {
    app_state: Arc<AppState>,
    current_user: Option<String>,
}

pub async fn graphql_handler(
    schema: Extension<schema::SafariSchema>,
    app_state: Extension<Arc<AppState>>,
    headers: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let user = match headers.get("x-user-name") {
        Some(name) => Some(String::from(name.to_str().unwrap())),
        None => None,
    };

    let gql_context = Arc::new(GqlContext {
        app_state: Arc::clone(&app_state),
        current_user: user,
    });

    let gql_executor = request
        .into_inner()
        .data(BatchCompanyById::new(&gql_context))
        .data(BatchUsersByCompanyId::new(&gql_context));

    schema.execute(gql_executor).await.into()
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
    .data(Arc::clone(&app_state))
    .finish()
}
