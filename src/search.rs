#![warn(
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
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

        let (location, date) = location_date.find(" - ").map_or_else(|| return (location_date.trim().to_string(), String::new()), |index| return (
                location_date[..index].trim().to_string(),
                location_date[index + 3..].trim().to_string(),
            ));

        let item = Item {
            url: format!("https://www.olx.bg{url}"),
            title,
            price,
            location,
            date,
        };
        items.push(item);
    }

    return items
}

fn make_request(query: String, min_price: String, max_price: String, page: String) -> Result<String, reqwest::Error> {
    // Base URL without the query
    let base_url = "https://www.olx.bg/ads/q-";

    // Build the query string with the provided parameters
    let query_string = format!(
        "{query}?page={page}search[filter_float_price:from]={min_price}&search[filter_float_price:to]={max_price}"
    );

    // Build the full URL with the base URL and query string
    let full_url = format!("{}{}", base_url, query_string);

    // Make the GET request
    let response = reqwest::blocking::get(&full_url)?;


    return Ok(response.text().unwrap())
}

pub fn new(query: String, min_price: String, max_price: String, mut page: u32) -> Vec<Item> {
    let mut items = Vec::new();

    loop {
        let html = match make_request(query.clone(), min_price.clone(), max_price.clone(), page.to_string()) {
            Ok(html) => html,
            Err(_) => {
                println!("[ ! ] Error fetching page {}", page);
                break; // Break the loop if there is an error fetching the page
            }
        };

        let parsed_items = parse_html(&html);
        items.extend(parsed_items);

        let has_next_page = html.contains("data-testid=\"pagination-forward\"");
        if !has_next_page {
            println!("[ - ] No next page after page {}", page);
            break; // Break the loop if there is no next page link
        }

        println!("[ + ] Went to page {}", page);
        page += 1; // Move to the next page
    }

    items
}