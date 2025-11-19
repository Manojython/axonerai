use crate::provider::{CompletionResponse, Message, Provider, StopReason, Tool, ToolCall};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize};
use serde_json::{json, Value};

pub struct AnthropicProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "claude-sonnet-4-20250514".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    async fn complete(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
        max_tokens: Option<u32>,
        system_prompt: Option<String>
    ) -> Result<CompletionResponse> {

        let mut body = json!({
            "system": system_prompt,
            "model": self.model,
            "max_tokens": max_tokens.unwrap_or(4096),
            "messages": messages,
        });

        // Add tools if provided
        if let Some(tools) = tools {
            let anthropic_tools: Vec<Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "input_schema": t.input_schema,
                    })
                })
                .collect();
            body["tools"] = json!(anthropic_tools);
        }

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("Anthropic API error {}: {}", status, error_text));
        }

        let api_response: AnthropicResponse = response.json().await?;
        
        // Parse the response into our unified format
        let mut text_parts = Vec::new();
        let mut tool_calls = Vec::new();

        for content in api_response.content {
            match content.content_type.as_str() {
                "text" => {
                    if let Some(text) = content.text {
                        text_parts.push(text);
                    }
                }
                "tool_use" => {
                    if let (Some(id), Some(name), Some(input)) = (content.id, content.name, content.input) {
                        tool_calls.push(ToolCall { id, name, input });
                    }
                }
                _ => {}
            }
        }

        let text = if text_parts.is_empty() {
            None
        } else {
            Some(text_parts.join("\n"))
        };

        let stop_reason = match api_response.stop_reason.as_str() {
            "tool_use" => StopReason::ToolUse,
            "end_turn" => StopReason::EndTurn,
            "max_tokens" => StopReason::MaxTokens,
            _ => StopReason::Error,
        };

        Ok(CompletionResponse {
            text,
            tool_calls,
            stop_reason,
        })
    }
}

// Anthropic API response structures
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    stop_reason: String,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<Value>,
}