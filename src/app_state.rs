use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn create(config: AppConfig, db: DatabaseConnection) -> Self {
        Self { config, db }
    }
}
