use crate::executor::{ToolExecutor, ToolResult};
use crate::provider::{Message, Provider, StopReason};
use crate::tool::ToolRegistry;
use anyhow::Result;
use crate::file_session_manager::FileSessionManager;
use crate::session::Session;

pub struct Agent {
    provider: Box<dyn Provider>,
    registry: ToolRegistry,
    max_iterations: usize,
    system_prompt: Option<String>,
    file_session_manager: Option<FileSessionManager>
}

impl Agent {
    pub fn new(provider: Box<dyn Provider>, registry: ToolRegistry, system_prompt: Option<String>, file_session_manager: Option<FileSessionManager>) -> Self {
        Self {
            provider,
            registry,
            max_iterations: 10, // Prevent infinite loops
            system_prompt,
            file_session_manager
        }
    }

    /// Run the agent with a user prompt
    pub async fn run(&self, user_prompt: &str) -> Result<String> {

        let mut session = if let Some(ref sm) = self.file_session_manager {
            if sm.exists() { sm.load()? }
            else { Session::new(sm.get_session().to_string()) }
        }
        else {
            Session::new("stateless".to_string())
        };



        println!();

        session.add_message(Message {
            role: "user".to_string(),
            content: user_prompt.to_string(),
        });

        let executor = ToolExecutor::new(&self.registry);
        let tools = self.registry.get_all_for_llm();

        for _iteration in 1..=self.max_iterations {

            let response = self
                .provider
                .complete(session.get_messages().clone(), Some(tools.clone()), None, self.system_prompt.clone())
                .await?;

            match response.stop_reason {
                StopReason::EndTurn => {
                    if let Some(text) = response.text {
                        session.add_message(Message {
                            role: "assistant".to_string(),
                            content: text.clone(),
                        });

                        if let Some(ref sm) = self.file_session_manager {
                            sm.save(&session)?;
                        }

                        println!("Response from Agent:");
                        return Ok(text);
                    } else {
                        return Ok("(No response from agent)".to_string());
                    }
                }

                StopReason::ToolUse => {
                    // LLM wants to use tools
                    println!("Entered here");
                    if let Some(text) = &response.text {
                        println!("💭 Agent thinking: {}", text);
                    }

                    if response.tool_calls.is_empty() {
                        return Ok("Agent wanted to use tools but didn't specify any".to_string());
                    }

                    // Execute the tools
                    let tool_results = executor.execute_all(&response.tool_calls).await?;

                    // Add assistant's tool use to messages
                    session.add_message(Message {
                        role: "assistant".to_string(),
                        content: format_tool_use(&response.tool_calls),
                    });

                    // Add tool results to messages
                    session.add_message(Message {
                        role: "user".to_string(),
                        content: format_tool_results(&tool_results),
                    });

                    println!();
                    // Continue the loop
                }

                StopReason::MaxTokens => {
                    return Ok("Agent hit max tokens limit".to_string());
                }

                _ => {
                    return Ok(format!("Agent stopped with reason: {:?}", response.stop_reason));
                }
            }
        }

        if let Some(ref sm) = self.file_session_manager {
            sm.save(&session)?;
        }
        Ok(format!(
            "Agent reached max iterations ({})",
            self.max_iterations
        ))
    }
}

fn format_tool_use(tool_calls: &[crate::provider::ToolCall]) -> String {
    tool_calls
        .iter()
        .map(|call| {
            format!(
                "Using tool '{}' with input: {}",
                call.name,
                serde_json::to_string(&call.input).unwrap_or_default()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format tool results for feeding back to the LLM
fn format_tool_results(results: &[ToolResult]) -> String {
    results
        .iter()
        .map(|result| format!("Tool '{}' returned: {}", result.tool_name, result.result))
        .collect::<Vec<_>>()
        .join("\n")
}