use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::*;
use async_graphql::*;
use sea_orm::*;

use crate::{
    app_state::AppState,
    entities::{company, user},
    graphql::{schema::user::BatchUsersByCompanyId, GqlContext},
};

#[ComplexObject]
impl company::Model {
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<BatchUsersByCompanyId>>();
        match loader.load_one(self.id).await {
            Ok(Some(users)) => Ok(users),
            _ => Ok(vec![]),
        }
    }
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

pub struct BatchCompanyById {
    ctx: Arc<GqlContext>,
}

impl BatchCompanyById {
    pub fn new(gql_context: &Arc<GqlContext>) -> DataLoader<Self> {
        DataLoader::new(
            Self {
                ctx: Arc::clone(gql_context),
            },
            tokio::spawn,
        )
    }
}

#[async_trait::async_trait]
impl Loader<i64> for BatchCompanyById {
    type Value = company::Model;
    type Error = FieldError;

    async fn load(&self, company_ids: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let companies = company::Entity::find()
            .filter(company::Column::Id.is_in(company_ids.to_owned()))
            .all(&self.ctx.app_state.db)
            .await?
            .into_iter()
            .map(|c| (c.id, c))
            .collect::<HashMap<i64, Self::Value>>();

        Ok(companies)
    }
}
