use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Define an async Agent trait; Send + Sync so we can put trait objects in Arc
#[async_trait]
pub trait Agent: Send + Sync {
    async fn run(&self, ctx: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn name(&self) -> &str;
}

// A simple LLM-backed agent stub
pub struct LlmAgent {
    name: String,
}
impl LlmAgent {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

#[async_trait]
impl Agent for LlmAgent {
    async fn run(&self, ctx: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // pretend to call an LLM
        println!("[LLM:{}] processing: {}", self.name(), ctx);
        sleep(Duration::from_millis(50)).await;
        println!("[LLM:{}] done", self.name());
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// A custom non-LLM agent
pub struct CustomAgent {
    name: String,
}
impl CustomAgent {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

#[async_trait]
impl Agent for CustomAgent {
    async fn run(&self, ctx: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("[Custom:{}] received: {}", self.name(), ctx);
        sleep(Duration::from_millis(30)).await;
        println!("[Custom:{}] finished", self.name());
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// A sequential agent that runs other agents in order
pub struct SequentialAgent {
    name: String,
    subagents: Vec<Arc<dyn Agent>>,
}
impl SequentialAgent {
    pub fn new(name: &str, subs: Vec<Arc<dyn Agent>>) -> Self {
        Self { name: name.to_string(), subagents: subs }
    }
}

#[async_trait]
impl Agent for SequentialAgent {
    async fn run(&self, ctx: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("[Seq:{}] starting sequence", self.name());
        for a in &self.subagents {
            println!("[Seq:{}] -> running sub-agent {}", self.name(), a.name());
            a.run(ctx).await?;
        }
        println!("[Seq:{}] sequence complete", self.name());
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Build some agents and store them as trait objects inside Arc
    let agents: Vec<Arc<dyn Agent>> = vec![
        Arc::new(LlmAgent::new("llm-main")),
        Arc::new(CustomAgent::new("custom-echo")),
        Arc::new(SequentialAgent::new(
            "pipeline-1",
            vec![
                Arc::new(LlmAgent::new("llm-sub")),
                Arc::new(CustomAgent::new("custom-sub")),
            ],
        )),
    ];

    // Run each agent. This works because all entries implement the Agent trait.
    for agent in agents {
        println!("=== Running agent: {} ===", agent.name());
        agent.run("Hello agent world").await?;
    }

    println!("All agents finished.");
    Ok(())
}
