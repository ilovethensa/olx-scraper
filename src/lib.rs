#![warn(
     clippy::nursery,
     clippy::suspicious,
     clippy::complexity,
     clippy::perf,
     clippy::style,
     clippy::panic,

 )]

pub mod item;
pub mod search;




/* #[test]
fn test_search() {
    let search_query: String = "iphone".to_owned();
    let min_price = "200.0";
    let max_price = "400.0";

    let html = make_request(search_query, min_price.to_owned(), max_price.to_owned()).unwrap();
    let items = parse_html(&html);

    for item in items {
        println!("URL: {}", item.url);
        println!("Title: {}", item.title);
        println!("Price: {}", item.price);
        println!();
    }
} */