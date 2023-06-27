use std::sync::Arc;

use async_graphql::*;
use sea_orm::*;

use crate::{
    app_state::AppState,
    entities::{company, user},
};

use super::company::CompanyByIdLoader;

#[ComplexObject]
impl user::Model {
    async fn company(&self, ctx: &Context<'_>) -> Result<company::Model, async_graphql::Error> {
        let loader = ctx.data_unchecked::<CompanyByIdLoader>();
        let company = loader.load(self.company_id).await;
        Ok(company)
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

// pub struct BatchUsersByCompanyId {
//     ctx: Arc<GqlContext>,
// }

// impl BatchUsersByCompanyId {
//     pub fn new(ctx: &Arc<GqlContext>) -> DataLoader<Self> {
//         DataLoader::new(
//             Self {
//                 ctx: Arc::clone(ctx),
//             },
//             tokio::spawn,
//         )
//     }
// }

// #[async_trait::async_trait]
// impl Loader<i64> for BatchUsersByCompanyId {
//     type Value = Vec<user::Model>;
//     type Error = FieldError;

//     async fn load(&self, company_ids: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
//         let users = user::Entity::find()
//             .filter(user::Column::CompanyId.is_in(company_ids.to_owned()))
//             .all(&self.ctx.app_state.db)
//             .await?;

//         let result = company_ids
//             .into_iter()
//             .map(|cid| {
//                 let company_users = users
//                     .iter()
//                     .filter(|u| u.company_id == *cid)
//                     .map(|u| u.to_owned())
//                     .collect::<Vec<user::Model>>();

//                 (*cid, company_users)
//             })
//             .collect::<HashMap<i64, Self::Value>>();

//         Ok(result)
//     }
// }
