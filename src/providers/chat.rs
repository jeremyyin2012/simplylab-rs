use std::cmp::max;
use std::str::FromStr;
use std::time::Duration;
use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::{DateTime as BsonDateTime, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::{FindOneOptions, FindOptions};
use crate::error::Error;

use crate::model::{Message, MessageDoc, MessageRoleType, NewMessage, User};
use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

pub struct ChatProvider {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}


impl ChatProvider {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl ChatProvider {
    pub async fn check_user_message_limited_in_30_seconds(&self, user: User) -> Result<bool, Error> {
        let now = Utc::now();
        let dt_start = now - Duration::from_secs(30);
        let start = BsonDateTime::from_chrono(now);
        let filter = doc! {
            "user_id": ObjectId::from_str(user.id.as_str()).with_context(||format!("parse oid error: {}", user.id))?,
            "type": MessageRoleType::User.to_string(),
            "created_at": {"$gte": start}
        };
        debug!("filter: {}", filter);
        let count = self.db.message().count_documents(filter, None).await
            .with_context(|| "count_documents".to_string())?;
        debug!("count: {}", count);
        if count > 3 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn check_user_message_limited_in_daily(&self, user: User) -> Result<bool, Error> {
        let now = Utc::now();
        let dt_start = NaiveDateTime::new(now.date_naive(), NaiveTime::default()).and_utc();
        let start = BsonDateTime::from_chrono(dt_start);
        let filter = doc! {
            "user_id": ObjectId::from_str(user.id.as_str()).with_context(||format!("parse oid error: {}", user.id))?,
            "type": MessageRoleType::User.to_string(),
            "created_at": {"$gte": start}
        };
        debug!("filter: {}", filter);
        let count = self.db.message().count_documents(filter, None).await
            .with_context(|| "count_documents".to_string())?;
        debug!("count: {}", count);
        if count > 20 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn add_chat_message(&self, messages: Vec<NewMessage>) -> Result<usize, Error> {
        let mut docs = vec![];
        for message in messages.iter() {
            let doc = MessageDoc {
                _id: ObjectId::new(),
                user_id: ObjectId::from_str(message.user_id.as_str()).with_context(||format!("parse oid error: {}", message.user_id))?,
                type_: message.type_.to_string(),
                text: message.text.to_owned(),
                created_at: BsonDateTime::now(),
                created_by: ObjectId::from_str(message.user_id.as_str()).with_context(||format!("parse oid error: {}", message.user_id))?,
                updated_at: None,
                updated_by: None,
            };
            docs.push(doc);
        }
        let res = self.db.message().insert_many(docs, None).await
            .with_context(|| "insert_many".to_string())?;
        debug!("inserted: {:?}", res);
        Ok(res.inserted_ids.len())
    }

    pub async fn get_user_chat_messages(&self, user: User, limit: i64) -> Result<Vec<Message>, Error> {
        let limit = max(limit, 10);
        let opts = FindOptions::builder().sort(doc! {"created_at": -1}).limit(limit).build();
        let filter = doc! {
            "user_id": ObjectId::from_str(user.id.as_str())?,
        };
        debug!("filter: {}", filter);
        let mut cursor = self.db.message().find(filter, opts).await
            .with_context(|| "find".to_string())?;
        let mut res = vec![];
        while let Some(doc) = cursor.try_next().await
            .with_context(|| "try_next".to_string())? {
            res.push(doc.to_entity()?)
        }
        debug!("messages: {:?}", res);
        Ok(res)
    }

    pub async fn get_user_chat_messages_count_today(&self, user: User) -> Result<u64, Error> {
        let now = Utc::now();
        let dt_start = NaiveDateTime::new(now.date_naive(), NaiveTime::default()).and_utc();
        let start = BsonDateTime::from_chrono(dt_start);
        let filter = doc! {
            "user_id": ObjectId::from_str(user.id.as_str()).with_context(||format!("parse oid error: {}", user.id))?,
            "type": MessageRoleType::User.to_string(),
            "created_at": {"$gte": start}
        };
        debug!("filter: {}", filter);
        let count = self.db.message().count_documents(filter, None).await
            .with_context(|| "count_documents".to_string())?;
        debug!("count: {}", count);
        Ok(count)
    }
}
