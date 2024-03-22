use crate::conf::Config;
use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;

pub mod api_client;
pub mod cache;
pub mod database;

#[derive(Clone)]
pub struct Store {
    pub config: Config,
    pub databases: Databases,
    pub caches: Caches,
    pub api_clients: ApiClients,
}

impl Store {
    pub async fn new() -> Self {
        let config = Config::new().await;
        Store {
            config: config.clone(),
            databases: Databases::new(config.clone()).await,
            caches: Caches::new(config.clone()).await,
            api_clients: ApiClients::new(config.clone()),
        }
    }
}
