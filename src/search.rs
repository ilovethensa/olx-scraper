use std::convert::TryInto;
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

fn parse_html(html: &str, selector: &Selector) -> Vec<Item> {
    let fragment = Html::parse_document(html);

    fragment
        .select(selector)
        .map(|element| {
            let url = format!("https://www.olx.bg{}", element.value().attr("href").unwrap_or_default());
            let title = element.select(&Selector::parse("h6.css-16v5mdi").unwrap()).next().map(|e| e.text().collect()).unwrap_or_default();
            let price = element.select(&Selector::parse("p.css-10b0gli").unwrap()).next().map(|e| e.text().collect()).unwrap_or_default();
            let location_date: String = element.select(&Selector::parse("p.css-veheph").unwrap()).next().map(|e| e.text().collect()).unwrap_or_default();
            let (location, date) = location_date.split_once(" - ").unwrap_or((location_date.trim(), ""));

            Item {
                url,
                title,
                price,
                location: location.trim().to_string(),
                date: date.trim().to_string(),
            }
        })
        .collect()
}

fn make_request(query: &str, category: Option<&str>, min_price: Option<&str>, max_price: Option<&str>, page: u32, sort: Option<&str>) -> Result<String, reqwest::Error> {
    let base_url = "https://www.olx.bg/ads/";
    let query_string = format!(
        "{}/q-{}?page={}",
        category.map_or_else(String::new, |cat| cat.to_string()),
        query,
        page
    );

    let query_string = if let Some(min) = min_price {
        format!("{}&search[filter_float_price:from]={}", query_string, min)
    } else {
        query_string
    };

    let query_string = if let Some(max) = max_price {
        format!("{}&search[filter_float_price:to]={}", query_string, max)
    } else {
        query_string
    };

    let query_string = if let Some(srt) = sort {
        let order = match srt {
            "1" => "relevance:desc",
            "2" => "relevance:asc",
            "3" => "created_at:desc",
            "4" => "created_at:asc",
            "5" => "filter_float_price:desc",
            "6" => "filter_float_price:asc",
            _ => std::process::exit(1), // Handle unknown sort option
        };
        format!("{}&search[order]={}", query_string, order)
    } else {
        query_string
    };

    let full_url = format!("{base_url}{query_string}", base_url = base_url, query_string = query_string).replace(' ', "-");

    let response = reqwest::blocking::get(full_url)?;
    let body = response.text().map_err(Into::into)?;

    Ok(body)
}

#[must_use]
pub fn new(query: String, category: Option<String>, min_price: Option<String>, max_price: Option<String>, end_page: Option<String>, sort: Option<&str>) -> Vec<Item> {
    let mut items = Vec::new();
    let mut current_page = 1;

    let selector = Selector::parse("a.css-rc5s2u").unwrap();

    while end_page.is_none() || (current_page <= end_page.as_deref().and_then(|s| s.parse().ok()).unwrap_or(std::usize::MAX)) {
        match make_request(&query, category.as_deref(), min_price.as_deref(), max_price.as_deref(), current_page.try_into().unwrap(), sort) {
            Ok(html) => {
                let parsed_items = parse_html(&html, &selector);
                items.extend(parsed_items);

                let has_next_page = html.contains("data-testid=\"pagination-forward\"");
                if !has_next_page {
                    println!("[ - ] No next page after page {}", current_page);
                    break;
                }

                println!("[ + ] Went to page {}", current_page);
                current_page += 1;
            }
            Err(err) => {
                println!("[ ! ] Error fetching page {}: {:?}", current_page, err);
                break;
            }
        }
    }

    items
}

