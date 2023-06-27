use std::{collections::HashMap, sync::Arc};

use async_graphql::*;
use dataloader::{non_cached::Loader, BatchFn};
use sea_orm::*;

use crate::{app_state::AppState, entities::company, graphql::GqlContext};

#[ComplexObject]
impl company::Model {
    pub async fn placeholder(&self) -> String {
        String::from("placeholder")
    }

    // pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, async_graphql::Error> {
    //     let loader = ctx.data_unchecked::<CompanyByIdLoader>();
    //     loader.load(self.id).await {
    //         Ok(Some(users)) => Ok(users),
    //         _ => Ok(vec![]),
    //     }
    // }
}

#[derive(Default)]
pub struct CompanyQueries;

#[Object]
impl CompanyQueries {
    pub async fn company_list(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<company::Model>, async_graphql::Error> {
        let app_state = ctx.data_unchecked::<Arc<AppState>>();
        let companies = company::Entity::find().limit(10).all(&app_state.db).await?;
        Ok(companies)
    }
}

pub struct CompanyByIdBatcher {
    ctx: Arc<GqlContext>,
}
impl CompanyByIdBatcher {
    pub fn new(gql_context: &Arc<GqlContext>) -> CompanyByIdLoader {
        Loader::new(Self {
            ctx: Arc::clone(gql_context),
        })
    }
}

#[async_trait::async_trait]
impl BatchFn<i64, company::Model> for CompanyByIdBatcher {
    async fn load(&mut self, company_ids: &[i64]) -> HashMap<i64, company::Model> {
        company::Entity::find()
            .filter(company::Column::Id.is_in(company_ids.to_owned()))
            .all(&self.ctx.app_state.db)
            .await
            .unwrap()
            .into_iter()
            .map(|c| (c.id, c))
            .collect::<HashMap<i64, company::Model>>()
    }
}

pub type CompanyByIdLoader = Loader<i64, company::Model, CompanyByIdBatcher>;
