use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::error::Error;

use crate::model::{Message, User};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDoc {
    pub _id: ObjectId,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

impl UserDoc {
    pub fn to_entity(self) -> Result<User, Error> {
        let user = User {
            id: self._id.to_hex(),
            name: self.name,
            created_at: self.created_at.to_chrono().naive_utc(),
            updated_at: if let Some(updated_at) = self.updated_at {
                Some(updated_at.to_chrono().naive_utc())
            } else { None },
        };
        Ok(user)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDoc {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    #[serde(rename="type")]
    pub type_: String,
    pub text: String,
    pub created_at: DateTime,
    pub created_by: ObjectId,
    pub updated_at: Option<DateTime>,
    pub updated_by: Option<ObjectId>,
}

impl MessageDoc {
    pub fn to_entity(self) -> Result<Message, Error> {
        let msg = Message {
            id: self._id.to_hex(),
            user_id: self.user_id.to_hex(),
            type_: self.type_.parse()?,
            text: self.text,
            created_at: self.created_at.to_chrono().naive_utc(),
            created_by: self.created_by.to_hex(),
            updated_at: if let Some(updated_at) = self.updated_at {
                Some(updated_at.to_chrono().naive_utc())
            } else { None },
            updated_by: if let Some(updated_by) = self.updated_by {
                Some(updated_by.to_hex())
            } else { None },
        };
        Ok(msg)
    }
}