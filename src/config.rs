pub struct Config {
    pub database_url: String,
}

impl Config {
    pub async fn build() -> Result<Self, std::env::VarError> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;

        Ok(Self { database_url })
    }
}
