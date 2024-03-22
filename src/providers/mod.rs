use crate::providers::ping::PingProvider;
use crate::providers::chat::ChatProvider;
use crate::providers::openrouter::OpenRouterProvider;
use crate::providers::user::UserProvider;
use crate::store::Store;

mod ping;
mod chat;
mod openrouter;
mod user;

#[derive(Clone)]
pub struct Providers {
    store: Store,
}

impl Providers {
    pub fn new(store: &Store) -> Self {
        Self {
            store: store.clone(),
        }
    }

    pub fn ping(&self) -> PingProvider {
        PingProvider::new(self.store.clone())
    }

    pub fn openrouter(&self) -> OpenRouterProvider {
        OpenRouterProvider::new(self.store.clone())
    }

    pub fn user(&self) -> UserProvider {
        UserProvider::new(self.store.clone())
    }

    pub fn chat(&self) -> ChatProvider {
        ChatProvider::new(self.store.clone())
    }
}
