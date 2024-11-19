mod adapters;

use anyhow::Result;
use std::fmt::Display;

pub use adapters::*;

pub enum AiAdapter {
    Ollama(OllamaAdapter),
}

#[derive(Debug)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[allow(async_fn_in_trait)]
pub trait AiService {
    async fn complete(&self, messages: &[Message]) -> Result<String>;
}

impl AiService for AiAdapter {
    async fn complete(&self, messages: &[Message]) -> Result<String> {
        match self {
            Self::Ollama(adapter) => adapter.complete(messages).await,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Assistant => write!(f, "assistant"),
            Self::System => write!(f, "system"),
        }
    }
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }
}
