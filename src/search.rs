#![warn(
    clippy::nursery,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::panic,
)]
use scraper::{Html, Selector};

#[derive(Debug)]
#[non_exhaustive]
pub struct Item {
    pub url: String,
    pub title: String,
    pub price: String,
    pub location: String,
    pub date: String,
}

fn parse_html(html: &str) -> Vec<Item> {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse("a.css-rc5s2u").unwrap();

    let mut items = Vec::new();

    for element in fragment.select(&selector) {
        let url = element.value().attr("href").unwrap_or_default().to_string();
        let title = element
            .select(&Selector::parse("h6.css-16v5mdi").unwrap())
            .next()
            .map(|e| return e.text().collect())
            .unwrap_or_default();

        let price = element
            .select(&Selector::parse("p.css-10b0gli").unwrap())
            .next()
            .map(|e| return e.text().collect())
            .unwrap_or_default();

        let location_date: String = element
            .select(&Selector::parse("p.css-veheph").unwrap())
            .next()
            .map(|e| return e.text().collect())
            .unwrap_or_default();

        let (location, date) = location_date.find(" - ").map_or_else(
            || {
                return (
                    location_date.trim().to_string(),
                    String::new(),
                )
            },
            |index| {
                return (
                    location_date[..index].trim().to_string(),
                    location_date[index + 3..].trim().to_string(),
                )
            },
        );

        let item = Item {
            url: format!("https://www.olx.bg{}", url),
            title,
            price,
            location,
            date,
        };
        items.push(item);
    }

    items
}

fn make_request(
    query: String,
    min_price: Option<String>,
    max_price: Option<String>,
    page: u32,
) -> Result<String, reqwest::Error> {
    // Base URL without the query
    let base_url = "https://www.olx.bg/ads/q-";

    // Build the query string with the provided parameters
    let mut query_string = format!("{query}?page={page}", query = query, page = page);

    if let Some(min) = min_price {
        query_string.push_str(&format!("&search[filter_float_price:from]={}", min));
    }

    if let Some(max) = max_price {
        query_string.push_str(&format!("&search[filter_float_price:to]={}", max));
    }

    // Build the full URL with the base URL and query string
    let full_url = format!("{base_url}{query_string}", base_url = base_url, query_string = query_string);

    // Make the GET request
    let response = reqwest::blocking::get(full_url)?;

    Ok(response.text().unwrap())
}

#[must_use]
pub fn new(
    query: String,
    min_price: Option<String>,
    max_price: Option<String>,
    end_page: u32,  // Rename the parameter to 'end_page'
) -> Vec<Item> {
    let mut items = Vec::new();

    let mut current_page = 1;  // Start from the first page

    while current_page <= end_page {  // Change the loop condition
        let html = match make_request(query.clone(), min_price.clone(), max_price.clone(), current_page) {
            Ok(html) => html,
            Err(_) => {
                println!("[ ! ] Error fetching page {}", current_page);
                break; // Break the loop if there is an error fetching the page
            }
        };

        let parsed_items = parse_html(&html);
        items.extend(parsed_items);

        let has_next_page = html.contains("data-testid=\"pagination-forward\"");
        if !has_next_page {
            println!("[ - ] No next page after page {}", current_page);
            break; // Break the loop if there is no next page link
        }

        println!("[ + ] Went to page {}", current_page);
        current_page += 1; // Move to the next page

        if current_page > end_page {
            break;  // Break the loop if the current page exceeds the specified end_page
        }
    }

    items
}