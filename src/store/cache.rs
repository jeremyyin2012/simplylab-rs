use std::env;

use redis::aio::ConnectionManager;
use redis::Client;
use crate::conf::Config;

#[derive(Clone)]
pub struct Caches {
    // pub default: ConnectionManager,
}

impl Caches {
    pub async fn new(config: Config) -> Self {
        println!("Caches init");
        Caches {
            // default: connect(config).await,
        }
    }
}

async fn connect(config: Config) -> ConnectionManager {
    let client = Client::open(config.redis_url).unwrap();
    ConnectionManager::new(client).await.unwrap()
}
