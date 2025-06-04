use fred::prelude::*;

pub struct CacheService {
    pool: RedisPool,
}

pub struct RedisPool {
    client: Client,
}

impl CacheService {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let config = Config::from_url(uri)?;
        let policy = ReconnectPolicy::new_exponential(0, 100, 30_000, 2);

        let client = Client::new(config, None, None, Some(policy));
        client.connect();
        client.wait_for_connect().await?;

        Ok(Self {
            pool: RedisPool { client },
        })
    }
}
