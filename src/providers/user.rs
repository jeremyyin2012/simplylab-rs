use anyhow::Context;
use chrono::{NaiveDateTime, Utc};
use mongodb::bson::{DateTime, doc};
use mongodb::bson::oid::ObjectId;
use crate::error::Error;
use crate::model::{User, UserDoc};
use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

pub struct UserProvider {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}


impl UserProvider {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl UserProvider {
    pub async fn get_user_by_name(self, user_name: String) -> Result<Option<User>, Error>{
        let user = self.db.user().find_one(doc! {"name": user_name.clone()}, None).await
            .with_context(|| format!("find_one by name: {}", user_name))?;
        if let Some(user) = user {
            Ok(Some(user.clone().to_entity().with_context(||format!("found user to_entity: {:?}", user))?))
        } else {
            let user = UserDoc {
                _id: ObjectId::new(),
                name: user_name,
                created_at: DateTime::now(),
                updated_at: None,
            };
            let res = self.db.user().insert_one(user, None).await
                .with_context(|| "insert_one".to_string())?;
            let user = self.db.user().find_one(doc! {"_id": res.inserted_id.clone()}, None).await
                .with_context(|| format!("find_one by _id {}", res.inserted_id))?;
            if let Some(user) = user {
                Ok(Some(user.clone().to_entity().with_context(||format!("new user to_entity: {:?}", user))?))
            } else {
                Ok(None)
            }
        }
    }
}