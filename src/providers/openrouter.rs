use anyhow::Context;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent, CompletionUsage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse, CreateCompletionRequestArgs, Role};
use reqwest::header::HeaderMap;
use rocket_okapi::hash_map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::store::api_client::ApiClients;
use crate::store::cache::Caches;
use crate::store::database::Databases;
use crate::store::Store;

pub struct OpenRouterProvider {
    store: Store,
    db: Databases,
    cache: Caches,
    api: ApiClients,
}


impl OpenRouterProvider {
    pub fn new(store: Store) -> Self {
        Self {
            store: store.clone(),
            db: store.databases.clone(),
            cache: store.caches.clone(),
            api: store.api_clients.clone(),
        }
    }
}

impl OpenRouterProvider {
    pub async fn chat(self, content: String) -> Result<String, Error> {
        // let config = OpenAIConfig::default()
        //     .with_api_base("https://openrouter.ai/api/v1")
        //     .with_api_key(self.store.config.openrouter_api_key);
        // let client = Client::with_config(config);
        // let request = CreateChatCompletionRequestArgs::default()
        //     .model("mistralai/mistral-7b-instruct:free")
        //     .messages(vec![
        //         ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        //             content: ChatCompletionRequestUserMessageContent::Text(content),
        //             role: Role::User,
        //             name: None,
        //         })
        //     ]).build().with_context(|| "build CreateChatCompletionRequestArgs".to_string())?;
        //
        // println!("{}", serde_json::to_string(&request).unwrap());
        // let response = client.chat().create(request).await
        //     .with_context(|| "chat create".to_string())?;
        let url = "https://openrouter.ai/api/v1/chat/completions";
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.store.config.openrouter_api_key).parse()?);
        headers.insert("Content-Type", "application/json".parse()?);
        let body = OpenRouterCreateChatCompletionRequestArgs {
            model: "mistralai/mistral-7b-instruct:free".to_string(),
            messages: vec![OpenRouterCreateChatCompletionRequestArgsMessage {
                role: Role::User,
                content: content,
            }],
        };
        let client = reqwest::Client::new();
        let response: OpenRouterCreateChatCompletionResponse = client.post(url).headers(headers).json(&body)
            .send().await.with_context(|| "send request to openrouter".to_string())?
            .json().await.with_context(|| "deserialize from openrouter".to_string())?;
        debug!("response: {:?}", response);
        let choice = response.choices[0].clone();
        if let Some(response_content) = choice.message.content {
            Ok(response_content)
        } else {
            Ok("todo".to_string())
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRouterCreateChatCompletionRequestArgsMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRouterCreateChatCompletionRequestArgs {
    pub model: String,
    pub messages: Vec<OpenRouterCreateChatCompletionRequestArgsMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRouterChatChoiceMessage {
    pub role: Role,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRouterChatChoice {
    pub message: OpenRouterChatChoiceMessage,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRouterCompletionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub total_cost: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenRouterCreateChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub created: u32,
    pub object: String,
    pub choices: Vec<OpenRouterChatChoice>,
    pub usage: Option<OpenRouterCompletionUsage>,
}