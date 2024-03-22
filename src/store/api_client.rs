use std::time::Duration;

use reqwest;
use url::Url;
use crate::conf::Config;
use crate::error::Error;

#[derive(Clone)]
pub struct ApiClients {
}

impl ApiClients {
    pub fn new(config: Config) -> Self {
        ApiClients {
        }
    }
}

#[derive(Clone)]
pub struct HttpClient {
    host: Url,
    location: String,
    pub client: reqwest::Client,
}

impl HttpClient {
    pub fn new(host: String, location: String) -> Self {
        let host = Url::parse(host.as_str()).expect("Invalid host");
        println!("{:?}", host);
        HttpClient {
            host,
            location,
            client: reqwest::Client::builder()
                .tcp_keepalive(Some(Duration::from_secs(3600)))
                .build()
                .expect("Build reqwest::Client failed"),
        }
    }
}

impl HttpClient {
    pub fn make_url(&self, path: String) -> Result<Url, Error> {
        let url = format!(
            "{}{}{}",
            self.host
                .as_str()
                .strip_suffix("/")
                .unwrap_or(self.host.as_str()),
            self.location.as_str(),
            path.as_str()
        );
        let url = Url::parse(url.as_str())?;
        println!("{:?}", url.to_string());
        Ok(url)
    }
}
