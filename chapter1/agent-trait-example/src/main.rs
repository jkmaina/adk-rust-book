use async_trait::async_trait;

#[async_trait]
pub trait Agent {
    async fn handle(&self, input: &str) -> String;
    fn name(&self) -> &str;
}

pub struct CustomerServiceAgent {
    agent_name: String,
}

impl CustomerServiceAgent {
    pub fn new(name: &str) -> Self {
        Self { agent_name: name.to_string() }
    }
}

#[async_trait]
impl Agent for CustomerServiceAgent {
    async fn handle(&self, input: &str) -> String {
        format!("Processing: {}", input)
    }

    fn name(&self) -> &str {
        &self.agent_name
    }
}

#[tokio::main]
async fn main() {
    let agent = CustomerServiceAgent::new("Alice");
    let response = agent.handle("I need help with my order").await;
    println!("Agent {} responded: {}", agent.name(), response);
}
