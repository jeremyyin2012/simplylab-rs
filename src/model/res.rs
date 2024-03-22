use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::model::MessageRoleType;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetAiChatResponseOutput {
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetChatStatusTodayOutput {
    pub user_name: String,
    pub chat_cnt: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserChatMessage {
    #[serde(rename="type")]
    pub type_: MessageRoleType,
    pub text: String,
}

pub type GetUserChatHistoryOutput = Vec<UserChatMessage>;