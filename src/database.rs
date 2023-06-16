use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn connect_to_database(connection_string: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(connection_string.to_owned());
    opt.max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public".into());

    let db = Database::connect(opt).await?;

    Ok(db)
}
