use async_graphql::{EmptySubscription, MergedObject, Object, Schema};

pub mod company;
pub mod user;

pub type SafariSchema = Schema<Query, MutationRoot, EmptySubscription>;

#[derive(Default, MergedObject)]
pub struct Query(QueryRoot, user::UserQueries, company::CompanyQueries);

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn world(&self) -> String {
        "world".to_owned()
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn goodbye(&self) -> String {
        String::from("Goodbye!")
    }
}
