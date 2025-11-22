use crate::provider::{CompletionResponse, Message, Provider, StopReason, Tool, ToolCall};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct GroqProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl GroqProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "openai/gpt-oss-20b".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl Provider for GroqProvider {
    async fn complete(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
        max_tokens: Option<u32>,
        system_prompt: Option<String>
    ) -> Result<CompletionResponse> {

        let mut complete_message:Vec<Value> = Vec::new();

        if let Some(sys_prompt) = system_prompt {
            complete_message.push(json!({
                "role": "system",
                "content": sys_prompt
            }));
        }

        let mut body = json!({
            "model": self.model,
            "messages": messages,
        });

        // let tool_clone = tools.clone();

        if let Some(max_tokens) = max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        // Add tools if provided (OpenAI-compatible format)
        if let Some(tools) = tools {
            let groq_tools: Vec<Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.input_schema,
                        }
                    })
                })
                .collect();
            body["tools"] = json!(groq_tools);
        }


        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("Groq API error {}: {}", status, error_text));
        }

        let api_response: GroqResponse = response.json().await?;
        
        // println!("DEBUG: Groq response: {}", serde_json::to_string_pretty(&api_response)?);


        let choice = api_response
            .choices
            .first()
            .ok_or_else(|| anyhow!("No choices in Groq response"))?;

        let text = choice.message.content.clone();

        let tool_calls = if let Some(calls) = &choice.message.tool_calls {
            calls
                .iter()
                .map(|tc| {
                    let input: Value = serde_json::from_str(&tc.function.arguments)
                        .unwrap_or(json!({}));
                    ToolCall {
                        id: tc.id.clone(),
                        name: tc.function.name.clone(),
                        input,
                    }
                })
                .collect::<Vec<ToolCall>>()
        } else {
            vec![]
        };

        let stop_reason = match choice.finish_reason.as_str() {
            "tool_calls" => StopReason::ToolUse,
            "stop" => StopReason::EndTurn,
            "length" => StopReason::MaxTokens,
            "content_filter" => StopReason::ContentFilter,
            _ => StopReason::Error,
        };

        Ok(CompletionResponse {
            text,
            tool_calls,
            stop_reason,
        })
    }
}

// Groq API response structures (OpenAI-compatible)
#[derive(Debug, Deserialize, Serialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    message: ResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseMessage {
    content: Option<String>,
    tool_calls: Option<Vec<GroqToolCall>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GroqToolCall {
    id: String,
    function: FunctionCall,
}

#[derive(Debug, Deserialize, Serialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}