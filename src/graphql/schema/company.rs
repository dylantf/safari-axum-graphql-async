use std::collections::HashMap;

use async_graphql::dataloader::*;
use async_graphql::*;
use sea_orm::*;

use crate::{
    entities::{company, user},
    graphql::schema::user::BatchUsersByCompanyId,
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
        let conn = ctx.data_unchecked::<DatabaseConnection>();
        let companies = company::Entity::find().limit(10).all(conn).await?;
        Ok(companies)
    }
}

pub struct BatchCompanyById {
    conn: DatabaseConnection,
}

impl BatchCompanyById {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl Loader<i64> for BatchCompanyById {
    type Value = company::Model;
    type Error = FieldError;

    async fn load(&self, company_ids: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let companies = company::Entity::find()
            .filter(company::Column::Id.is_in(company_ids.to_owned()))
            .all(&self.conn)
            .await?
            .into_iter()
            .map(|c| (c.id, c))
            .collect::<HashMap<i64, Self::Value>>();

        Ok(companies)
    }
}
