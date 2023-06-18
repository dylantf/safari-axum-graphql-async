use std::collections::HashMap;

use async_graphql::dataloader::*;
use async_graphql::*;
use sea_orm::*;

use crate::{
    entities::{company, user},
    graphql::schema::user::BatchUsersByCompanyId,
    AppState,
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

pub struct BatchCompanyById(pub AppState);

#[async_trait::async_trait]
impl Loader<i64> for BatchCompanyById {
    type Value = company::Model;
    type Error = FieldError;

    async fn load(&self, company_ids: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let companies = company::Entity::find()
            .filter(company::Column::Id.is_in(company_ids.to_owned()))
            .all(&self.0.db)
            .await?
            .into_iter()
            .map(|c| (c.id, c))
            .collect::<HashMap<i64, Self::Value>>();

        Ok(companies)
    }
}
