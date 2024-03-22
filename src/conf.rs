use std::env;

use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_env: String,
    pub debug: bool,
    pub sentry_dsn: String,
    pub database_url: String,
    pub max_size: u32,
    pub redis_url: String,
    pub openrouter_api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_env: "dev".to_string(),
            debug: true,
            sentry_dsn: "".to_string(),
            database_url: "".to_string(),
            max_size: 10,
            redis_url: "".to_string(),
            openrouter_api_key: "".to_string(),
        }
    }
}

impl Config {
    pub async fn new() -> Self {
        println!("Config init");
        let config = connect().await;
        println!("{:?}", config);
        config
    }
}

async fn connect() -> Config {
    dotenv().ok();
    let app_env = env::var("APP_ENV").unwrap_or("dev".to_string());

    let debug = env::var("DEBUG").unwrap_or("true".to_string())
        .parse::<bool>()
        .unwrap();

    let sentry_dsn = env::var("SENTRY_DSN").unwrap_or("".to_string());

    let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());
    let max_size = env::var("DATABASE_POOL_MAX_SIZE").unwrap_or("10".to_string())
        .parse::<u32>()
        .unwrap();

    let redis_url = env::var("REDIS_URL").unwrap_or("".to_string());
    let openrouter_api_key = env::var("OPENROUTER_API_KEY").unwrap_or("".to_string());

    Config {
        app_env,
        debug,
        sentry_dsn,
        database_url,
        max_size,
        redis_url,
        openrouter_api_key,
        ..Default::default()
    }
}
