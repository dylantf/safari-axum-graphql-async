use async_graphql::{
    dataloader::DataLoader, http::GraphiQLSource, EmptySubscription, MergedObject, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};

use self::schema::{CompanyLoader, UserQueries};
use crate::AppState;

mod schema;

#[derive(Default)]
pub struct BaseQueries;

#[Object]
impl BaseQueries {
    pub async fn hello(&self) -> String {
        String::from("Hello!")
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn goodbye(&self) -> String {
        String::from("Goodbye!")
    }
}

#[derive(Default, MergedObject)]
pub struct Query(BaseQueries, UserQueries);

pub type SeshtrackerSchema = Schema<Query, MutationRoot, EmptySubscription>;

pub fn build_graphql_schema(app_state: AppState) -> SeshtrackerSchema {
    Schema::build(Query::default(), MutationRoot, EmptySubscription)
        .data(DataLoader::new(
            CompanyLoader {
                app_state: app_state.clone(),
            },
            tokio::spawn,
        ))
        .data(app_state)
        .finish()
}

pub async fn graphql_handler(
    schema: Extension<SeshtrackerSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

pub async fn graphiql_handler() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/api/graphql").finish())
}
