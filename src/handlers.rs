use axum::{extract::State, response::IntoResponse, Json};
use sea_orm::*;
use serde::Serialize;
use serde_json::json;

use crate::{
    entities::{company, user},
    AppState,
};

#[derive(Serialize)]
struct UserListItem<'a> {
    user: &'a user::Model,
    company: Option<&'a company::Model>,
}

pub async fn user_list_handler(State(app_state): State<AppState>) -> impl IntoResponse {
    let users = user::Entity::find()
        .limit(10)
        .all(&app_state.db)
        .await
        .unwrap();

    let company_ids = users.iter().map(|u| u.company_id);

    let companies = company::Entity::find()
        .filter(company::Column::Id.is_in(company_ids))
        .all(&app_state.db)
        .await
        .unwrap();

    let response: Vec<UserListItem> = users
        .iter()
        .map(|user| UserListItem {
            user: &user,
            company: companies.iter().find(|c| c.id == user.company_id),
        })
        .collect();

    Json(json!(response))
}
