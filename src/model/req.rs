use chrono::NaiveDateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use rocket::form::FromForm;
use crate::model::MessageRoleType;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, FromForm)]
pub struct GetAiChatResponseInput {
    pub message: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetUserChatHistoryInput {
    pub user_name: String,
    pub last_n: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetChatStatusTodayInput {
    pub user_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewMessage {
    pub user_id: String,
    #[serde(rename="type")]
    pub type_: MessageRoleType,
    pub text: String,
}

