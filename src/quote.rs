use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct QuoteResponse {
    content: String,
    author: String,
}

pub fn fetch_quote() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.quotable.io/random";
    let response = get(url)?.json::<QuoteResponse>()?;
    Ok(format!("\"{}\" - {}", response.content, response.author))
}
