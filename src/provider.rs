use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::Result;

/// Core trait that all LLM providers must implement
#[async_trait]
pub trait Provider: Send + Sync {
    /// Send a completion request to the LLM
    async fn complete(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
        max_tokens: Option<u32>,
        system_prompt: Option<String>
    ) -> Result<CompletionResponse>;
}

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,      // "user" or "assistant"
    pub content: String,
}

/// Tool definition following OpenAI/Anthropic function calling spec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,  // JSON schema for the tool's parameters
}

/// Unified response from any LLM provider
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub text: Option<String>,           // Text response (if any)
    pub tool_calls: Vec<ToolCall>,      // Tool calls (EMPTY VEC if none, NOT Option)
    pub stop_reason: StopReason,        // Why it stopped
}

/// When the model wants to call a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub input: Value,
}

/// Why the completion stopped
#[derive(Debug, Clone, PartialEq)]
pub enum StopReason {
    EndTurn,        // Natural completion
    ToolUse,        // Wants to call tools
    MaxTokens,      // Hit token limit
    ContentFilter,  // Filtered by provider
    Error,          // Something went wrong
}