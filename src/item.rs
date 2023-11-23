#![warn(
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
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

pub fn get(url: &str) -> Result<AdDetails, reqwest::Error> {
    // Make an HTTP GET request to the specified URL
    let body = other_get(url)?.text().unwrap();

    // Parse the HTML content
    let document = Html::parse_document(&body);

    // Define selectors for the elements we want to extract
    let title_selector = Selector::parse(".css-1juynto").map_err(|_| "Error parsing title selector").unwrap();
    let price_selector = Selector::parse(".css-12vqlj3").map_err(|_| "Error parsing price selector").unwrap();
    let description_selector = Selector::parse(".css-1t507yq").map_err(|_| "Error parsing description selector").unwrap();
    let user_selector = Selector::parse("div.css-1ucpzm6:nth-child(1) > a:nth-child(1)").map_err(|_| "Error parsing user selector").unwrap();
    let date_selector = Selector::parse(".css-19yf5ek").map_err(|_| "Error parsing date selector").unwrap();

    // Extract data using the selectors
    let title = document.select(&title_selector).next().map(|e| e.text().collect());
    let price = document.select(&price_selector).next().map(|e| e.text().collect());
    let description = document.select(&description_selector).next().map(|e| e.text().collect());
    let user = document.select(&user_selector).next().and_then(|e| e.value().attr("href")).map(|href| href.to_string()).ok_or("Error getting href").unwrap();
    let date = document.select(&date_selector).next().map(|e| e.text().collect()).ok_or("Error getting date").unwrap();

    // Create and return an AdDetails struct
    Ok(AdDetails {
        title: title.unwrap_or_else(|| "N/A".to_string()),
        price: price.unwrap_or_else(|| "N/A".to_string()),
        description: description.unwrap_or_else(|| "N/A".to_string()),
        user: format!("https://www.olx.bg{}", user),
        date,
    })
}
