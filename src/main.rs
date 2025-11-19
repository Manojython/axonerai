use std::time::Instant;
use axonai::agent::Agent;
use axonai::provider::{Provider};
use std::env;
use axonai::anthropic::AnthropicProvider;
use axonai::groq::GroqProvider;
use axonai::tool::ToolRegistry;
use axonai::tools::calculator::Calculator;
use axonai::tools::{WebScrape, WebSearch};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    println!("Starting AxonAI example...");

    let provider_type = env::var("PROVIDER_TYPE").unwrap_or("groq".to_string());

    let provider: Box<dyn Provider> = match provider_type.as_str() {
        "anthropic" => {
            let api_key = env::var("ANTHROPIC_API_KEY")
                .expect("ANTHROPIC_API_KEY environment variable not set");
            Box::new(AnthropicProvider::new(api_key))
        },
        "groq" | _ => {
            let api_key = env::var("GROQ_API_KEY")
                .expect("GROQ_API_KEY environment variable not set");
            Box::new(GroqProvider::new(api_key))
        },
    };

    let mut tools = ToolRegistry::new();
    tools.register(Box::new(Calculator));
    tools.register(Box::new(WebSearch));
    tools.register(Box::new(WebScrape));
    println!("Available tools: {:?}", &tools.list_tools());
    let system_prompt = "You are a helpful assistant, You have several tools at you disposal, \
    do not give information without proper usage of tools, you are smart, but you rely on tools for information".to_string();
    let agent = Agent::new(provider, tools, Some(system_prompt));



    let mut input = String::new();
    println!("Enter your message: ");
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered: {}", input.trim());
    let start_time = Instant::now();
    let response = agent.run(input.trim()).await?;
    // let messages = vec![
    //     Message{
    //         role: "user".to_string(),
    //         content: input.trim().to_string(),
    //     }
    // ];
    let elapsed = start_time.elapsed();
    println!("{}", &response.to_string());
    println!("Time elapsed is: {:?}", elapsed);

    
    // let response = provider.complete(messages, Some(tools.get_all_for_llm()), None).await?;

    // if let Some(text) = response.text {
    //     println!("Response: {}", text);
    // } else {
    //     println!("No text response received.");
    // }

    Ok(())
    
}
