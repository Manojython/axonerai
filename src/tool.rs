use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {

    fn name(&self) -> String;

    fn description(&self) -> String;

    fn input_schema(&self) -> Value;

    async fn execute(&self, input: Value) -> Result<String>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name();
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }

    pub fn get_all_for_llm(&self) -> Vec<crate::provider::Tool> {
        self.tools
            .values()
            .map(|tool| crate::provider::Tool {
                name: tool.name(),
                description: tool.description(),
                input_schema: tool.input_schema(),
            })
            .collect()
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}