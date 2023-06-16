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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: i32,
    name: String,
    username: String,
    company_id: i32,
}

#[derive(SimpleObject)]
pub struct Company {
    id: i32,
    name: String,
}

impl From<user::Model> for User {
    fn from(u: user::Model) -> Self {
        Self {
            id: u.id as i32,
            name: u.name,
            username: u.username,
            company_id: u.company_id as i32,
        }
    }
}

impl From<company::Model> for Company {
    fn from(c: company::Model) -> Self {
        Self {
            id: c.id as i32,
            name: c.name,
        }
    }
}

#[ComplexObject]
impl User {
    async fn company(&self, ctx: &Context<'_>) -> Result<Company, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<CompanyLoader>>();
        let company: Option<company::Model> = loader.load_one(self.company_id).await?;
        match company {
            Some(c) => Ok(c.into()),
            None => Err(async_graphql::Error::new("Err")),
        }
    }
}

pub struct CompanyLoader {
    pub app_state: AppState,
}

#[async_trait::async_trait]
impl Loader<i32> for CompanyLoader {
    type Value = company::Model;
    type Error = FieldError;

    async fn load(&self, company_ids: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let results = company::Entity::find()
            .filter(company::Column::Id.is_in(company_ids.to_owned()))
            .all(&self.app_state.db)
            .await
            .unwrap();

        let mut ret: HashMap<i32, Self::Value> = HashMap::new();
        for r in results {
            ret.insert(r.id as i32, r);
        }

        Ok(ret)
    }
}

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    pub async fn user_list(&self, ctx: &Context<'_>) -> Result<Vec<User>, DbErr> {
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
