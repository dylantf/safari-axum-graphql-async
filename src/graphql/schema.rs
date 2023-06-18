use std::collections::HashMap;

use async_graphql::{
    dataloader::{DataLoader, Loader},
    *,
};
// use chrono::{DateTime, Utc};
use sea_orm::*;

use crate::{
    entities::{company, user},
    AppState,
};

#[ComplexObject]
impl user::Model {
    async fn company(&self, ctx: &Context<'_>) -> Result<company::Model, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<CompanyLoader>>();
        let company: Option<company::Model> = loader.load_one(self.company_id).await?;
        match company {
            Some(c) => Ok(c),
            None => Err(async_graphql::Error::new("Err")),
        }
    }
}

pub struct CompanyLoader {
    pub app_state: AppState,
}

#[async_trait::async_trait]
impl Loader<i64> for CompanyLoader {
    type Value = company::Model;
    type Error = FieldError;

    async fn load(&self, company_ids: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let results = company::Entity::find()
            .filter(company::Column::Id.is_in(company_ids.to_owned()))
            .all(&self.app_state.db)
            .await
            .unwrap();

        let mut ret: HashMap<i64, Self::Value> = HashMap::new();
        for r in results {
            ret.insert(r.id, r);
        }

        Ok(ret)
    }
}

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    pub async fn user_list(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, DbErr> {
        let app = ctx.data_unchecked::<AppState>();

        let users = user::Entity::find()
            .limit(10)
            .all(&app.db)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(users)
    }
}
