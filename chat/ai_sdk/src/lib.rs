mod adapters;

use std::fmt::Display;

use anyhow::Result;

pub use adapters::*;

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

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Assistant => write!(f, "assistant"),
            Self::System => write!(f, "system"),
        }
    }
}
