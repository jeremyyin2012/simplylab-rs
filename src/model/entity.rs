use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::error::Error;
use bson::serde_helpers::hex_string_as_object_id;


pub type UserId = String;
pub type UserName = String;
pub type CreatedAt = NaiveDateTime;
pub type CreatedBy = UserId;
pub type UpdatedAt = Option<NaiveDateTime>;
pub type UpdatedBy = Option<UserId>;


#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub enum MessageRoleType {
    #[serde(rename="user")]
    User,
    #[serde(rename="ai")]
    AI,
}

impl FromStr for MessageRoleType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Self::User),
            "ai" => Ok(Self::AI),
            _ => Err(Error::ParamsError("ai/user pls".to_string()))
        }
    }
}

impl Display for MessageRoleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => f.write_str("user"),
            Self::AI => f.write_str("ai"),
        }
    }
}



pub type MessageId = String;
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Message {
    pub id: MessageId,
    pub user_id: UserId,
    #[serde(rename="type")]
    pub type_: MessageRoleType,
    pub text: String,
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub updated_at: UpdatedAt,
    pub updated_by: UpdatedBy,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Context {
    pub user: User,
}

impl Context {
    pub fn new(user: User) -> Self {
        Self {
            user
        }
    }
}

