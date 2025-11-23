use std::path::PathBuf;
use std::time::Instant;
use axonerai::agent::Agent;
use axonerai::provider::{Provider};
use std::env;
use axonerai::anthropic::AnthropicProvider;
use axonerai::openai::OpenAIProvider;
use axonerai::groq::GroqProvider;
use axonerai::tool::ToolRegistry;
use axonerai::tools::{WebScrape, WebSearch, Calculator};
use axonerai::file_session_manager::FileSessionManager;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let provider_type = env::var("PROVIDER_TYPE").unwrap_or("groq".to_string());

    let provider: Box<dyn Provider> = match provider_type.as_str() {
        "anthropic" => {
            let api_key = env::var("ANTHROPIC_API_KEY")
                .expect("ANTHROPIC_API_KEY environment variable not set");
            Box::new(AnthropicProvider::new(api_key))
        },
        "openai" => {
            let api_key = env::var("OPENAI_API_KEY")
                .expect("OPENAI_API_KEY environment variable not set");
            Box::new(OpenAIProvider::new(api_key))
        },
        "groq" | _ => {
            let api_key = env::var("GROQ_API_KEY")
                .expect("GROQ_API_KEY environment variable not set");
            Box::new(GroqProvider::new(api_key))
        },
    };

    /*
    -------- Tool Registry----------------
     */
    let mut tools = ToolRegistry::new();
    tools.register(Box::new(Calculator));
    tools.register(Box::new(WebSearch));
    tools.register(Box::new(WebScrape));
    println!("Available tools: {:?}", &tools.list_tools());

    /*
    ----------Session Manager------------
     */
    let session_id = Uuid::new_v4().to_string();
    let file_session_manager = FileSessionManager::new(session_id , PathBuf::from("../tmp/"))?;


    /* Agent Initialize */
    let system_prompt = "You are a helpful assistant, You have several tools at you disposal, \
    do not give information without proper usage of tools, you are smart, but you rely on tools \
    for information".to_string();
    let agent = Agent::new(
        provider,
        tools,
        Some(system_prompt),
        Some(file_session_manager)
    );

    /*
    ------- Agent Loop ------------
    */
    let mut input = String::new();
    println!("🤖 Agent starting...");
    loop {
        println!(" You 🦁: ");
        println!("---------------");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if &input.trim().to_lowercase() == "quit" || &input.trim().to_lowercase() == "exit" {
            println!("Bye!👋");
            break;
        }
        let start_time = Instant::now();
        let response = agent.run(input.trim()).await?;
        let elapsed = start_time.elapsed();
        println!("----------------------------------------");
        println!("{}", &response.to_string());
        println!("Time taken for response: {:?}", elapsed);
    }


    Ok(())

}
