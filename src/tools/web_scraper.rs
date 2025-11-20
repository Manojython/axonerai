use crate::tool::Tool;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest;
use scraper::{Html, Selector};
use std::time::Instant;


pub struct WebScrape;

#[derive(Debug, Deserialize, Serialize)]
struct WebScrapeInput {
    titles: Vec<String>,
    links: Vec<String>,
}
#[async_trait]
impl Tool for WebScrape {


    fn name(&self) -> String{ "WebScrape".to_string()}

    fn description(&self) -> String{ "This tool gives the actual content of the page and returns text for which the user has asked".to_string()}

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "titles": {
                    "type": "array",
                    "description": "array of titles from the search",
                    "items": {"type": "string"}
                },
                "links": {
                    "type": "array",
                    "description": "array of links from the search",
                    "items": {"type": "string"}
                }
            },
            "required": ["titles", "links"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let input: WebScrapeInput = serde_json::from_value(input)
            .map_err(|e| anyhow!("Invalid calculator input: {}", e))?;

        println!("{:?}", input);
        let titles = input.titles;
        let links = input.links;
        let mut search_blob = "Title, WebpageContent".to_string();
        let web_scr_start = Instant::now();
        for (title, link ) in titles.iter().zip(links.iter()) {
            let content = fetch_content(link.to_string()).await?;
            println!("{}",content);
            search_blob.push_str(&format!("\n{}: {}", title, content));
        }
        println!("Time taken for web scraping: {:?}", web_scr_start.elapsed());
        Ok(search_blob)

    }
}
async fn fetch_html(url: &str) -> Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
async fn parse_html_content(html_body: &str) -> Result<String> {
    let document = Html::parse_document(html_body);
    let selector = Selector::parse("p, h1, h2, h3, h4").unwrap();
    let mut content = String::new();
    for element in document.select(&selector) {
        content.push_str(&element.inner_html());
    }
    Ok(content)
}
async fn fetch_content(link: String) -> Result<String> {
    let html_body = fetch_html(&link).await?;
    Ok(parse_html_content(&html_body).await?)
}