use ai_sdk::{AiAdapter, AiService, OllamaAdapter};
use chat_core::{
    AdapterType, Agent, AgentContext, AgentDecision, AgentError, AgentType, ChatAgent,
};

pub enum AgentVariant {
    Proxy(ProxyAgent),
    Reply(ReplyAgent),
    Tap(TapAgent),
}

#[allow(unused)]
pub struct ProxyAgent {
    pub name: String,
    pub adapter: AiAdapter,
    pub prompt: String,
    pub args: serde_json::Value,
}

#[allow(unused)]
pub struct ReplyAgent {
    pub name: String,
    pub adapter: AiAdapter,
    pub prompt: String,
    pub args: serde_json::Value,
}

#[allow(unused)]
pub struct TapAgent {
    pub name: String,
    pub adapter: AiAdapter,
    pub prompt: String,
    pub args: serde_json::Value,
}

impl Agent for ProxyAgent {
    async fn process(&self, msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        let prompt = format!("{} {}", self.prompt, msg);
        let messages = vec![ai_sdk::Message::user(prompt)];
        let res = self.adapter.complete(&messages).await?;
        Ok(AgentDecision::Modify(res))
    }
}

impl Agent for ReplyAgent {
    async fn process(&self, msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        let prompt = format!("{} {}", self.prompt, msg);
        let messages = vec![ai_sdk::Message::user(prompt)];
        let res = self.adapter.complete(&messages).await?;
        Ok(AgentDecision::Reply(res))
    }
}

impl Agent for TapAgent {
    async fn process(&self, _msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        Ok(AgentDecision::None)
    }
}

impl Agent for AgentVariant {
    async fn process(&self, msg: &str, ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        match self {
            AgentVariant::Proxy(agent) => agent.process(msg, ctx).await,
            AgentVariant::Reply(agent) => agent.process(msg, ctx).await,
            AgentVariant::Tap(agent) => agent.process(msg, ctx).await,
        }
    }
}

impl From<ChatAgent> for AgentVariant {
    fn from(mut value: ChatAgent) -> Self {
        let adapter = match value.adapter {
            AdapterType::Ollama => AiAdapter::Ollama(OllamaAdapter::new_local(value.model)),
        };
        match value.r#type {
            AgentType::Reply => AgentVariant::Reply(ReplyAgent {
                name: value.name,
                adapter,
                prompt: value.prompt,
                args: value.args.take(),
            }),
            AgentType::Proxy => AgentVariant::Proxy(ProxyAgent {
                name: value.name,
                adapter,
                prompt: value.prompt,
                args: value.args.take(),
            }),
            AgentType::Tap => AgentVariant::Tap(TapAgent {
                name: value.name,
                adapter,
                prompt: value.prompt,
                args: value.args.take(),
            }),
        }
    }
}

impl From<ProxyAgent> for AgentVariant {
    fn from(value: ProxyAgent) -> Self {
        AgentVariant::Proxy(value)
    }
}

impl From<ReplyAgent> for AgentVariant {
    fn from(value: ReplyAgent) -> Self {
        AgentVariant::Reply(value)
    }
}

impl From<TapAgent> for AgentVariant {
    fn from(value: TapAgent) -> Self {
        AgentVariant::Tap(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppState;
    use anyhow::Result;

    #[ignore]
    #[tokio::test]
    async fn agent_variant_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let agents = state.list_agents(1).await?;
        let agent = agents[0].clone();
        let agent: AgentVariant = agent.into();
        let decision = agent.process("hello", &AgentContext::default()).await?;
        if let AgentDecision::Modify(_content) = decision {
        } else {
            panic!("decision is not modify");
        }
        Ok(())
    }
}
