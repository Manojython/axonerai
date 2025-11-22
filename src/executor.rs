use crate::provider::ToolCall;
use crate::tool::ToolRegistry;
use anyhow::{anyhow, Result};

/// Executes tool calls and returns results
pub struct ToolExecutor<'a> {
    registry: &'a ToolRegistry,
}

impl<'a> ToolExecutor<'a> {
    pub fn new(registry: &'a ToolRegistry) -> Self {
        Self { registry }
    }

    /// Execute a single tool call
    pub async fn execute(&self, tool_call: &ToolCall) -> Result<ToolResult> {
        println!("  🔧 Executing tool: {}", tool_call.name);
        
        let tool = self
            .registry
            .get(&tool_call.name)
            .ok_or_else(|| anyhow!("Tool not found: {}", tool_call.name))?;

        let result = tool.execute(tool_call.input.clone()).await?;

        Ok(ToolResult {
            tool_call_id: tool_call.id.clone(),
            tool_name: tool_call.name.clone(),
            result,
        })
    }

    /// Execute multiple tool calls
    
    pub async fn execute_all(&self, tool_calls: &[ToolCall]) -> Result<Vec<ToolResult>> {
        let mut results = Vec::new();
        for call in tool_calls {
            results.push(self.execute(call).await?);
        }
        Ok(results)
    }
}

/// Result from executing a tool
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub tool_name: String,
    pub result: String,
}