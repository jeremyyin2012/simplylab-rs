use std::fmt::format;
use anyhow::Context as AnyhowContext;
use chrono::{NaiveDateTime, Utc};
use log::debug;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use redis::ToRedisArgs;
use crate::error::{Code, Error};
use crate::model::{Context, GetAiChatResponseInput, GetAiChatResponseOutput, GetChatStatusTodayOutput, GetUserChatHistoryOutput, Message, MessageRoleType, NewMessage, UserChatMessage};
use crate::providers::Providers;

pub struct ChatService {
    ctx: Context,
    pvd: Providers,
}

impl ChatService {
    pub fn new(context: Context, providers: Providers) -> Self {
        Self {
            ctx: context,
            pvd: providers,
        }
    }
}


impl ChatService {
    pub async fn get_ai_chat_response(&self, req: GetAiChatResponseInput) -> Result<GetAiChatResponseOutput, Error> {
        let limited = self.pvd.chat().check_user_message_limited_in_30_seconds(self.ctx.user.clone()).await
            .with_context(||format!("check_user_message_limited_in_30_seconds: {:?}", self.ctx.user.clone()))?;
        if limited {
            return Err(Error::Unauthorized);
        }
        let limited = self.pvd.chat().check_user_message_limited_in_daily(self.ctx.user.clone()).await
            .with_context(||format!("check_user_message_limited_in_daily: {:?}", self.ctx.user.clone()))?;
        if limited {
            return Err(Error::Unauthorized);
        }

        let request_content = req.message;
        // todo: request conent middle out
        let response_content = self.pvd.openrouter().chat(request_content.clone()).await
            .with_context(|| format!("chat: {}", request_content.clone()))?;
        let now = Utc::now();
        let created_at = NaiveDateTime::new(now.date_naive(), now.time());
        let user_message = NewMessage {
            user_id: self.ctx.user.id.to_string(),
            type_: MessageRoleType::User,
            text: request_content.to_string(),
        };
        let ai_message = NewMessage {
            user_id: self.ctx.user.id.to_string(),
            type_: MessageRoleType::AI,
            text: response_content.to_string(),
        };
        let messages = vec![user_message, ai_message];
        let count = self.pvd.chat().add_chat_message(messages).await
            .with_context(|| "add_chat_message".to_string())?;
        debug!("Added {count} chat messages");
        let res = GetAiChatResponseOutput {
            response: response_content,
        };
        Ok(res)
    }

    pub async fn get_user_chat_history(&self, last_n: i64) -> Result<GetUserChatHistoryOutput, Error> {
        let messages = self.pvd.chat().get_user_chat_messages(self.ctx.user.clone(), last_n).await
            .with_context(||format!("get_user_chat_messages: {:?}", self.ctx.user.clone()))?;
        let mut res = vec![];
        for msg in messages.iter() {
            res.push(UserChatMessage {
                type_: msg.type_.clone(),
                text: msg.text.clone(),
            });
        }
        Ok(res)
    }

    pub async fn get_chat_status_today(&self) -> Result<GetChatStatusTodayOutput, Error> {
        let count = self.pvd.chat().get_user_chat_messages_count_today(self.ctx.user.clone()).await
            .with_context(||format!("get_user_chat_messages_count_today: {:?}", self.ctx.user.clone()))?;
        let res = GetChatStatusTodayOutput {
            user_name: self.ctx.user.name.clone(),
            chat_cnt: count,
        };
        Ok(res)
    }
}