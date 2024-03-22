use sqlx::Connection;
use crate::error::Error;

use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

pub struct PingProvider {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}

impl PingProvider {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl PingProvider {
    pub async fn ping_mysql(&self) -> Result<String, Error> {
        // let mut conn = self.db.user();
        // conn.ping().await?;
        Ok("pong".to_string())
    }

    pub async fn ping_pgsql(&self) -> Result<String, Error> {
        // let mut conn = self.db.default.acquire().await?;
        // conn.ping().await?;
        Ok("pong".to_string())
    }

    pub async fn ping_redis(&self) -> Result<String, Error> {
        // let mut conn = self.cache.default.clone();
        // let reply: RedisResult<String> = redis::cmd("PING").query_async(&mut conn).await;
        // assert_eq!("PONG", reply.unwrap());
        Ok("pong".to_string())
    }
}
