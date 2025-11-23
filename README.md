# AxonerAI

A type-safe, high-performance agentic AI framework in Rust.

**Why AxonerAI?**
- 🦀 **Type-safe** - Catch errors at compile time, not runtime
- ⚡ **Fast** - Native Rust performance, no Python overhead
- 🔧 **Simple** - Clean API, minimal boilerplate
- 📦 **Single binary** - No dependency hell, easy deployment

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
axonerai = "0.1"
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4"] }
```

## Quick Start

```rust
use axonerai::agent::Agent;
use axonerai::groq::GroqProvider;
use axonerai::tool::ToolRegistry;
use axonerai::tools::{Calculator, WebSearch};
use axonerai::file_session_manager::FileSessionManager;
use std::path::PathBuf;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Create a provider
    let provider = GroqProvider::new(
        std::env::var("GROQ_API_KEY")?,
        "llama-3.3-70b-versatile".to_string(),
    );

    // 2. Register tools
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(Calculator));
    registry.register(Box::new(WebSearch::new()));

    // 3. Create session manager (optional - pass None for stateless)
    let session_manager = FileSessionManager::new(
        uuid::Uuid::new_v4().to_string(),
        PathBuf::from("./sessions"),
    )?;

    // 4. Create agent
    let system_prompt = "You are a helpful assistant. You have several tools at your disposal. \
        Do not give information without proper usage of tools.".to_string();
    
    let agent = Agent::new(
        Box::new(provider),
        registry,
        Some(system_prompt),
        Some(session_manager), // Or None for stateless
    );

    // 5. Interactive loop
    let mut input = String::new();
    println!("🤖 Agent starting... (type 'quit' to exit)");
    
    loop {
        println!("\nYou 🦁: ");
        println!("---------------");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim().to_lowercase() == "quit" || input.trim().to_lowercase() == "exit" {
            println!("Bye! 👋");
            break;
        }

        let start_time = Instant::now();
        let response = agent.run(input.trim()).await?;
        let elapsed = start_time.elapsed();
        
        println!("----------------------------------------");
        println!("{}", response);
        println!("Time taken: {:?}", elapsed);
        
        input.clear();
    }

    Ok(())
}
```

## Supported Providers

| Provider | Model Examples                                        |
|----------|-------------------------------------------------------|
| **Groq** | `llama-3.3-70b-versatile`, `gpt-oss-20B`              |
| **Anthropic** | `claude-sonnet-4-20250514`, `claude-3-haiku-20240307` |
| **OpenAI** | `gpt-4o`, `gpt-4o-mini`                               |

```rust
// Groq (free tier available!)
use axonerai::groq::GroqProvider;
let provider = GroqProvider::new(api_key, "llama-3.3-70b-versatile".to_string());

// Anthropic
use axonerai::anthropic::AnthropicProvider;
let provider = AnthropicProvider::new(api_key, "claude-sonnet-4-20250514".to_string());

// OpenAI
use axonerai::openai::OpenAIProvider;
let provider = OpenAIProvider::new(api_key, "gpt-4o".to_string());
```

## Built-in Tools

- **Calculator** - Basic arithmetic operations
- **WebSearch** - Search the web via Google Custom Search API
- **WebScraper** - Scrape content from URLs

## Creating Custom Tools

```rust
use axonerai::tool::{Tool, ToolResult};
use serde_json::{json, Value};
use anyhow::Result;

pub struct MyTool;

impl Tool for MyTool {
    fn name(&self) -> String {
        "my_tool".to_string()
    }

    fn description(&self) -> String {
        "Does something useful".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "The input to process"
                }
            },
            "required": ["input"]
        })
    }

    fn execute(&self, input: Value) -> Result<String> {
        let input_str = input["input"].as_str().unwrap_or("");
        Ok(format!("Processed: {}", input_str))
    }
}
```

Register it:

```rust
use axonerai::tool::ToolRegistry;

let mut registry = ToolRegistry::new();
registry.register(Box::new(MyTool));
```

## Environment Variables

```bash
# Required for your chosen provider
GROQ_API_KEY=your_groq_key
ANTHROPIC_API_KEY=your_anthropic_key
OPENAI_API_KEY=your_openai_key

# For WebSearch tool
GOOGLE_API_KEY=your_google_key
GOOGLE_CX=your_search_engine_id
```

## Features

- [x] Multi-provider support (Groq, Anthropic, OpenAI)
- [x] Tool system with custom tool support
- [x] Session management (file-based)
- [x] System prompts

## Comparison with Python Frameworks

| Feature | AxonerAI | LangChain |
|---------|----------|----------|
| Language | Rust | Python |
| Type Safety |  Compile-time |  Runtime |
| Performance |  Fast |  Slow |
| Learning Curve | Moderate | Easy |

## Responsible Use

**Important:** AxonerAI provides tools for web search and web scraping. Users are responsible for ensuring their use complies with applicable laws, terms of service, and ethical guidelines.

### Web Tools Guidelines

**WebSearch Tool:**
- Requires a Google Custom Search API key and Search Engine ID
- Subject to [Google's Custom Search JSON API Terms of Service](https://developers.google.com/custom-search/v1/overview)
- Respect rate limits (100 queries/day on free tier)
- Commercial use may require paid quota

**WebScraper Tool:**
- Always respect `robots.txt` directives
- Check website Terms of Service before scraping
- Implement appropriate rate limiting to avoid overloading servers
- Some websites explicitly prohibit automated scraping
- Consider legal implications in your jurisdiction

### API Provider Terms

When using AxonerAI with LLM providers, you must comply with their respective terms:
- [Anthropic Terms of Service](https://www.anthropic.com/legal/terms)
- [OpenAI Terms of Use](https://openai.com/policies/terms-of-use)
- [Groq Terms of Service](https://groq.com/terms-of-service/)

### Your Responsibility

By using AxonerAI, you agree that:
- You are responsible for your use of the framework and its tools
- You will comply with all applicable laws and third-party terms of service
- The maintainers of AxonerAI are not liable for misuse of the framework

### Use responsibly. Respect the web.

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR on GitHub.