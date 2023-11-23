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

// ... (previous code)

pub fn get(url: &str) -> Result<AdDetails, Box<dyn std::error::Error>> {
    // Make an HTTP GET request to the specified URL
    let body = other_get(url)?.text()?;

    // Parse the HTML content
    let document = Html::parse_document(&body);

    // Define selectors for the elements we want to extract
    let title_selector =
        Selector::parse(".css-1juynto").map_err(|e| format!("Error parsing title selector: {e}"))?;
    let price_selector =
        Selector::parse(".css-12vqlj3").map_err(|e| format!("Error parsing price selector: {e}"))?;
    let description_selector = Selector::parse(".css-1t507yq")
        .map_err(|e| format!("Error parsing description selector: {e}"))?;
    let user_selector = Selector::parse("div.css-1ucpzm6:nth-child(1) > a:nth-child(1)")
        .map_err(|e| format!("Error parsing user selector: {e}"))?;
    let date_selector =
        Selector::parse(".css-19yf5ek").map_err(|e| format!("Error parsing date selector: {e}"))?;

    // Extract data using the selectors
    let title = document.select(&title_selector).next().map(|e| e.text().collect());
    let price = document.select(&price_selector).next().map(|e| e.text().collect());
    let description = document
        .select(&description_selector)
        .next()
        .map(|e| e.text().collect());
    let user = document
        .select(&user_selector)
        .next()
        .and_then(|e| e.value().attr("href"))
        .map(|href| format!("https://www.olx.bg{href}"))
        .ok_or("Error getting href")?;
    let date = document.select(&date_selector).next().map(|e| e.text().collect()).ok_or("Error getting date")?;

    // Create and return an AdDetails struct
    Ok(AdDetails {
        title: title.unwrap_or_else(|| "N/A".to_string()),
        price: price.unwrap_or_else(|| "N/A".to_string()),
        description: description.unwrap_or_else(|| "N/A".to_string()),
        user,
        date,
    })
}
