use crate::provider::{CompletionResponse, Message, Provider, StopReason, Tool, ToolCall};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct OpenAIProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: "gpt-5-mini".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn complete(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<Tool>>,
        max_completion_tokens: Option<u32>,
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

        if let Some(max_completion_tokens) = max_completion_tokens {
            body["max_completion_tokens"] = json!(max_completion_tokens);
        }

        // Add tools if provided
        if let Some(tools) = tools {
            let openai_tools: Vec<Value> = tools
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
            body["tools"] = json!(openai_tools);
        }



        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-type", "application/json")
            .json(&body)
            .send()
            .await?;


        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow!("OpenAI API error {}: {}", status, error_text));
        }

        let api_response: OpenAIResponse = response.json().await?;


        let choice = api_response
            .choices
            .first()
            .ok_or_else(|| anyhow!("No choices in OpenAI response"))?;

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

        println!("DEBUG: stop_reason: {:?}\n", &stop_reason);

        Ok(CompletionResponse {
            text,
            tool_calls,
            stop_reason,
        })
    }
}

// OpenAI API response structures
#[derive(Debug, Deserialize, Serialize)]
struct OpenAIResponse {
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
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenAIToolCall {
    id: String,
    function: FunctionCall,
}

#[derive(Debug, Deserialize, Serialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}