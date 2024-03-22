use crate::error::Error;
use crate::model::Context;
use crate::providers::Providers;

pub struct PingService {
    ctx: Context,
    pvd: Providers,
}

impl PingService {
    pub fn new(context: Context, providers: Providers) -> Self {
        Self {
            ctx: context,
            pvd: providers,
        }
    }
}

impl PingService {
    pub async fn do_ping(&self) -> Result<String, Error> {
        self.pvd.ping().ping_pgsql().await?;
        self.pvd.ping().ping_redis().await?;
        Ok("pong".to_string())
    }
    pub async fn do_ping_mysql(&self) -> Result<String, Error> {
        self.pvd.ping().ping_mysql().await?;
        Ok("pong".to_string())
    }
    pub async fn do_ping_pgsql(&self) -> Result<String, Error> {
        self.pvd.ping().ping_pgsql().await?;
        Ok("pong".to_string())
    }
    pub async fn do_ping_redis(&self) -> Result<String, Error> {
        self.pvd.ping().ping_redis().await?;
        Ok("pong".to_string())
    }
}
