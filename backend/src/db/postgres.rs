use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Error,
};
use std::time::Duration;

pub struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .acquire_timeout(Duration::from_secs(3))
            .connect(uri)
            .await?;
        Ok(Self { pool })
    }
}
