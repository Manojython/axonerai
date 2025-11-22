# AxonerAI

A type-safe, high-performance agentic AI framework in Rust.

**Why AxonerAI?**
- 🦀 **Type-safe** - Catch errors at compile time, not runtime
- ⚡ **Fast** - Native Rust performance, no Python overhead
- 🔧 **Simple** - Clean API, minimal boilerplate
- 📦 **Single binary** - No dependency hell, easy deployment

## Supported Providers

| Provider | Model Examples                                        |
|----------|-------------------------------------------------------|
| **Groq** | `llama-3.3-70b-versatile`, `gpt-oss-20B`              |
| **Anthropic** | `claude-sonnet-4-20250514`, `claude-3-haiku-20240307` |
| **OpenAI** | `gpt-5.1-mini`, `gpt-4o-mini`                         |

```rust
// Groq (free tier available!)
let provider = GroqProvider::new(api_key, "llama-3.3-70b-versatile".to_string());

// Anthropic
let provider = AnthropicProvider::new(api_key, "claude-sonnet-4-20250514".to_string());

// OpenAI
let provider = OpenAIProvider::new(api_key, "gpt-4o".to_string());
```

## Built-in Tools

- **Calculator** - Basic arithmetic operations
- **WebSearch** - Search the web via Google Custom Search API
- **WebScraper** - Scrape content from URLs
