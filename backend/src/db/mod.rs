use cache::CacheService;
use postgres::PostgresRepo;

pub mod cache;
pub mod postgres;

pub struct Database {
    pub cache: CacheService,
    pub postgres: PostgresRepo,
}

impl Database {
    pub async fn new(pg_uri: &str, redis_uri: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            cache: CacheService::new(redis_uri).await?,
            postgres: PostgresRepo::new(pg_uri).await?,
        })
    }
}
