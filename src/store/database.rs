use std::env;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection, Database};
use crate::conf::Config;
use crate::error::Error;
use crate::model::{MessageDoc, UserDoc};

#[derive(Clone, Debug)]
pub struct Databases {
    pub default: Database,
}

impl Databases {
    pub async fn new(config: Config) -> Self {
        println!("Databases init");
        let db = Databases {
            default: connect(config).await.expect("can not connect to mongodb."),
        };
        println!("{db:?}");
        db
    }

    pub fn user(&self) -> Collection<UserDoc> {
        return self.default.collection::<UserDoc>("user")
    }

    pub fn message(&self) -> Collection<MessageDoc> {
        return self.default.collection::<MessageDoc>("message")
    }
}

pub async fn connect(config: Config) -> mongodb::error::Result<Database> {
    let mongo_host = env::var("MONGO_HOST").unwrap_or("localhost".to_string());
    let mongo_port = env::var("MONGO_PORT").unwrap_or("27017".to_string());
    let mongo_username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME is not set in .env");
    let mongo_password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD is not set in .env");
    // mongodb://{MONGO_USERNAME}:{MONGO_PASSWORD}@mongodb:27017/
    let mongo_uri = format!("mongodb://{mongo_username}:{mongo_password}@{mongo_host}:{mongo_port}/");
    println!("mongo_uri: {mongo_uri}");
    let mongo_db_name = env::var("MONGO_DB_NAME").unwrap_or("simplylab".to_string());

    let client_options = ClientOptions::parse(mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let dbs = client.list_databases(None, None).await?;
    println!("databases: {dbs:?}");
    let database = client.database(mongo_db_name.as_str());
    Ok(database)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Databases {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Error> {
        let db = request.rocket().state::<Databases>();
        match db {
            Some(db) => request::Outcome::Success(db),
            None => request::Outcome::Error((
                Status::ServiceUnavailable,
                Error::DatabaseConnectionError("从 State 获取连接池失败".to_string()),
            )),
        }
    }
}
