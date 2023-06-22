use tracing::level_filters::LevelFilter;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub log_level: LevelFilter,
}

impl AppConfig {
    pub async fn create() -> Result<Self, std::env::VarError> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;

        let log_level = match std::env::var("LOG_LEVEL") {
            Ok(level) => match level.as_str() {
                "trace" => LevelFilter::TRACE,
                "debug" => LevelFilter::DEBUG,
                "info" => LevelFilter::INFO,
                "warn" => LevelFilter::WARN,
                "error" => LevelFilter::ERROR,
                &_ => {
                    println!("Invalid log level, expected debug, info, trace, warn, error");
                    LevelFilter::ERROR
                }
            },
            Err(_) => LevelFilter::ERROR,
        };

        Ok(Self {
            database_url,
            log_level,
        })
    }
}
