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
        let url = format!("https://www.olx.bg{}", element.value().attr("href").unwrap_or_default());
        let title = element
            .select(&Selector::parse("h6.css-16v5mdi").unwrap())
            .next()
            .map(|e| e.text().collect())
            .unwrap_or_default();

        let price = element
            .select(&Selector::parse("p.css-10b0gli").unwrap())
            .next()
            .map(|e| e.text().collect())
            .unwrap_or_default();

        let location_date: String = element
            .select(&Selector::parse("p.css-veheph").unwrap())
            .next()
            .map(|e| e.text().collect())
            .unwrap_or_default();
        let temp = String::new();
        let (location, date) = location_date.split_once(" - ").unwrap_or((
            location_date.trim(),
            &temp,
        ));

        let item = Item {
            url,
            title,
            price,
            location: location.trim().to_string(),
            date: date.trim().to_string(),
        };
        items.push(item);
    }

    items
}

fn make_request(
    query: &str,
    category: Option<&str>, // Add optional category parameter
    min_price: Option<&str>,
    max_price: Option<&str>,
    page: u32,
) -> Result<String, reqwest::Error> {
    let base_url = "https://www.olx.bg/ads/";
    let mut query_string = String::new();

    if let Some(cat) = category {
        query_string.push_str(cat);
    }
    query_string.push_str(&format!("q-{query}", query = query));


    query_string.push_str(&format!("?page={page}", page = page));

    if let Some(min) = min_price {
        query_string.push_str(&format!("&search[filter_float_price:from]={}", min));
    }

    if let Some(max) = max_price {
        query_string.push_str(&format!("&search[filter_float_price:to]={}", max));
    }

    let full_url = format!("{base_url}{query_string}", base_url = base_url, query_string = query_string);

    let response = reqwest::blocking::get(&full_url)?;

    Ok(response.text().unwrap())
}

#[must_use]
pub fn new(
    query: String,
    category: Option<String>, // Change to Option<String> for category
    min_price: Option<String>,
    max_price: Option<String>,
    end_page: u32,
) -> Vec<Item> {
    let mut items = Vec::new();
    let mut current_page = 1;

    while current_page <= end_page {
        match make_request(&query, category.as_deref(), min_price.as_deref(), max_price.as_deref(), current_page) {
            Ok(html) => {
                let parsed_items = parse_html(&html);
                items.extend(parsed_items);

                let has_next_page = html.contains("data-testid=\"pagination-forward\"");
                if !has_next_page {
                    println!("[ - ] No next page after page {}", current_page);
                    break;
                }

                println!("[ + ] Went to page {}", current_page);
                current_page += 1;

                if current_page > end_page {
                    break;
                }
            }
            Err(_) => {
                println!("[ ! ] Error fetching page {}", current_page);
                break;
            }
        }
    }

    items
}
