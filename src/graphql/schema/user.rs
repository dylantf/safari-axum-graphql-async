use std::{collections::HashMap, sync::Arc};

use async_graphql::dataloader::*;
use async_graphql::*;
use sea_orm::*;

use crate::{
    app_state::AppState,
    entities::{company, user},
    graphql::schema::company::BatchCompanyById,
};

#[ComplexObject]
impl user::Model {
    async fn company(&self, ctx: &Context<'_>) -> Result<company::Model, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<BatchCompanyById>>();
        let company: Option<company::Model> = loader.load_one(self.company_id).await?;
        match company {
            Some(c) => Ok(c),
            None => Err(async_graphql::Error::new("Err")),
        }
    }
}

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    pub async fn user_list(&self, ctx: &Context<'_>) -> Result<Vec<user::Model>, DbErr> {
        let app_state = ctx.data_unchecked::<Arc<AppState>>();
        let users = user::Entity::find().limit(10).all(&app_state.db).await?;
        Ok(users)
    }
}

pub struct BatchUsersByCompanyId {
    app_state: Arc<AppState>,
}

impl BatchUsersByCompanyId {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
}

#[async_trait::async_trait]
impl Loader<i64> for BatchUsersByCompanyId {
    type Value = Vec<user::Model>;
    type Error = FieldError;

    async fn load(&self, company_ids: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let users = user::Entity::find()
            .filter(user::Column::CompanyId.is_in(company_ids.to_owned()))
            .all(&self.app_state.db)
            .await?;

        let result: HashMap<i64, Self::Value> =
            company_ids
                .into_iter()
                .fold(HashMap::new(), |mut acc, cid| {
                    let company_users = users
                        .iter()
                        .filter(|u| u.company_id == *cid)
                        .map(|u| u.to_owned())
                        .collect::<Vec<user::Model>>();

                    acc.insert(*cid, company_users);
                    acc
                });

        Ok(result)
    }
}
