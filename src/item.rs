#![warn(
    clippy::nursery,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::panic,
)]

use scraper::{Html, Selector};
use reqwest::blocking::get as other_get;

#[derive(Debug)]
#[non_exhaustive]
pub struct AdDetails {
    pub title: String,
    pub price: String,
    pub description: String,
    pub user: String,
    pub date: String,
}

fn parse_selector(selector: &str, body: &Html) -> Result<String, String> {
    Selector::parse(selector)
        .map_err(|e| format!("Error parsing {} selector: {}", selector, e)).map(|s| body.select(&s).next().map(|e| e.text().collect()).unwrap_or_else(|| "N/A".to_string()))
}

pub fn get(url: &str) -> Result<AdDetails, Box<dyn std::error::Error>> {
    let body = other_get(url)?.text()?;
    let document = Html::parse_document(&body);

    Ok(AdDetails {
        title: parse_selector(".css-1juynto", &document)?,
        price: parse_selector(".css-12vqlj3", &document)?,
        description: parse_selector(".css-1t507yq", &document)?,
        user: document
            .select(&Selector::parse("div.css-1ucpzm6:nth-child(1) > a:nth-child(1)").unwrap())
            .next()
            .and_then(|e| e.value().attr("href"))
            .map(|href| format!("https://www.olx.bg{href}"))
            .ok_or("Error getting href")?,
        date: parse_selector(".css-19yf5ek", &document)?,
    })
}
