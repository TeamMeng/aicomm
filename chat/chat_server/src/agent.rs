use ai_sdk::{AiAdapter, AiService, OllamaAdapter};
use chat_core::{
    AdapterType, Agent, AgentContext, AgentDecision, AgentError, AgentType, ChatAgent,
};

pub enum AgentVariant {
    Proxy(ProxyAgent),
    Reply(ReplyAgent),
    Tap(TapAgent),
    Test(TestAgent),
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

#[allow(unused)]
pub struct TestAgent;

impl Agent for ProxyAgent {
    async fn process(&self, msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        // If we need it to be flexible: prompt is a jinja2 template, and args is a json
        let prompt = format!("{} {}", self.prompt, msg);
        let messages = vec![ai_sdk::Message::user(prompt)];
        let res = self.adapter.complete(&messages).await?;
        Ok(AgentDecision::Modify(res))
    }
}

impl Agent for ReplyAgent {
    async fn process(&self, msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        // 1. create embedding for the message
        // 2. search related docs via vector db with embedding
        //let docs = searcher.search(msg).await?;
        // 3. query llm with prompt and related docs as context
        // let prompt = format!("{} {} {}", self.prompt, docs, msg);
        let prompt = format!("{} {}", self.prompt, msg);
        let messages = vec![ai_sdk::Message::user(prompt)];
        let res = self.adapter.complete(&messages).await?;
        Ok(AgentDecision::Reply(res))
    }
}

// in future, we should push messages into a queue and process in a delayed manner
impl Agent for TapAgent {
    async fn process(&self, _msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        Ok(AgentDecision::None)
    }
}

impl Agent for TestAgent {
    async fn process(&self, _msg: &str, _ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        Ok(AgentDecision::Modify("test".to_string()))
    }
}

impl Agent for AgentVariant {
    async fn process(&self, msg: &str, ctx: &AgentContext) -> Result<AgentDecision, AgentError> {
        match self {
            AgentVariant::Reply(agent) => agent.process(msg, ctx).await,
            AgentVariant::Proxy(agent) => agent.process(msg, ctx).await,
            AgentVariant::Tap(agent) => agent.process(msg, ctx).await,
            AgentVariant::Test(agent) => agent.process(msg, ctx).await,
        }
    }
}

impl From<ChatAgent> for AgentVariant {
    fn from(mut agent: ChatAgent) -> Self {
        let adapter: AiAdapter = match agent.adapter {
            AdapterType::Ollama => OllamaAdapter::new_local(agent.model).into(),
            AdapterType::Test => return AgentVariant::Test(TestAgent),
        };

        match agent.r#type {
            AgentType::Reply => AgentVariant::Reply(ReplyAgent {
                name: agent.name,
                adapter,
                prompt: agent.prompt,
                args: agent.args.take(),
            }),
            AgentType::Proxy => AgentVariant::Proxy(ProxyAgent {
                name: agent.name,
                adapter,
                prompt: agent.prompt,
                args: agent.args.take(),
            }),
            AgentType::Tap => AgentVariant::Tap(TapAgent {
                name: agent.name,
                adapter,
                prompt: agent.prompt,
                args: agent.args.take(),
            }),
        }
    }
}

impl From<ProxyAgent> for AgentVariant {
    fn from(agent: ProxyAgent) -> Self {
        AgentVariant::Proxy(agent)
    }
}

impl From<ReplyAgent> for AgentVariant {
    fn from(agent: ReplyAgent) -> Self {
        AgentVariant::Reply(agent)
    }
}

impl From<TapAgent> for AgentVariant {
    fn from(agent: TapAgent) -> Self {
        AgentVariant::Tap(agent)
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
        let agents = state.list_agents(1).await.expect("list agents failed");
        let agent = agents[0].clone();
        let agent: AgentVariant = agent.into();
        let decision = agent.process("hello", &AgentContext::default()).await?;
        // test if it is modify
        if let AgentDecision::Modify(_content) = decision {
        } else {
            panic!("decision is not modify");
        }
        Ok(())
    }
}
