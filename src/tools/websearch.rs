use crate::tool::Tool;
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest;
pub struct WebSearch;

#[derive(Debug, Deserialize, Serialize)]
struct WebSearchInput {
    search_term: String
}
#[async_trait]
impl Tool for WebSearch {


    fn name(&self) -> String{ "WebSearch".to_string()}

    fn description(&self) -> String{ "This tool uses Google Search and returns some links".to_string()}

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "search_term": {
                    "type": "string",
                    "description": "query text"
                }
            },
            "required": ["search_term"]
        })
    }

   async fn execute(&self, input: Value) -> Result<String> {
        let input: WebSearchInput = serde_json::from_value(input)
            .map_err(|e| anyhow!("Invalid calculator input: {}", e))?;

        let search_api_key = env::var("SEARCH_API_KEY")?;
        let cx = env::var("CX_ENGINE")?;
        let url = "https://www.googleapis.com/customsearch/v1?".to_string();
        let response = reqwest::Client::new()
            .get(url)
            .query(&[
                ("key", search_api_key),
                ("cx", cx),
                ("q", input.search_term),
                ("num", "3".to_string())
            ])
            .send()
            .await?;

        let data: Value = response.json().await?;

        Ok(process_json_data(&data)?)

    }
}

fn process_json_data(data: &Value) -> Result<String> {

    let items: &Vec<Value> = data["items"].as_array().ok_or(anyhow!("Missing items"))?;
    let mut search_results:String = "title, link, description_preview".to_string();
    //filter(|item| item.key.contains("title") || item.key.contains("link") || item.key.contains("snippet") )
    for item in items {

        search_results.push_str("\n");
        search_results.push_str(&format!("{},{},{}", &item["title"], &item["link"], &item["snippet"]));

        println!("Matched item: Title: {}", &item["title"]);
    }
    Ok(search_results)
}