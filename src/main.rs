#![warn(
     clippy::nursery,
     clippy::suspicious,
     clippy::complexity,
     clippy::perf,
     clippy::style,
     clippy::panic,

 )]
use clap::{Command, Arg};
use olx::{search, item};

fn main() {
    let matches = Command::new("OLX Tool")
        .version("0.1.0")
        .author("TheHolyTachanka")
        .subcommand(
            Command::new("search")
                .about("Searches OLX for products within a price range.")
                .arg(Arg::new("query").short('q').help("The search query").required(true))
                .arg(Arg::new("min_price").short('m').help("The minimum price").required(false))
                .arg(Arg::new("max_price").short('x').help("The maximum price").required(false))
                .arg(Arg::new("category").short('c').help("The category to search in").required(false))
                .arg(Arg::new("page").short('p').help("On which page to end the search").required(false))
                .arg(Arg::new("sort").short('s').help("What to sort by").required(false))
        )
        .subcommand(
            Command::new("get")
                .about("Gets information about an item from its URL.")
                .arg(Arg::new("url").help("The URL of the item").required(true)),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("search") => {
            let search_matches = matches.subcommand_matches("search").unwrap();
            let search_query = search_matches.get_one::<String>("query").map(|s| s.to_string());
            let min_price = search_matches.get_one::<String>("min_price").map(|s| s.to_string());
            let max_price = search_matches.get_one::<String>("max_price").map(|s| s.to_string());
            let category = search_matches.get_one::<String>("category").map(|s| s.to_string());
            let page = search_matches.get_one::<String>("page").map(|s| s.to_string());
            let sort = search_matches.get_one::<String>("sort").map(|s| s.to_string());
            let sort_str = sort.as_deref().unwrap_or_default();

            let mut lol = String::new();
            match sort_str {
                "relevance_desc" => lol.push('1'),
                "relevance_asc" => lol.push('2'),
                "created_at_desc" => lol.push('3'),
                "created_at_asc" => lol.push('4'),
                "price_desc" => lol.push('5'),
                "price_asc" => lol.push('6'),
                _ => {
                    lol.push('1')
                }
            }
            // Call your search function with the provided arguments
            let items = search::new(
                search_query.unwrap_or_else(|| "default_query".to_string()),
                category,
                min_price,
                max_price,
                page,
                Some(lol.as_str())

            );
            let json = search::to_json(items);
            println!("{json}");
        }
        Some("get") => {
            let get_matches = matches.subcommand_matches("get").unwrap();
            let item_url = get_matches.get_one::<String>("url").unwrap().to_string();

            match item::get(&item_url) {
                Ok(info) => {
                    println!("{info:#?}");
                    // Print or process the information
                }
                Err(err) => {
                    eprintln!("Error getting item information: {err}");
                    // Handle the error as needed, e.g., return early or display a user-friendly message.
                }
            }
        }
        _ => println!("Invalid subcommand. Use 'search' or 'get'."),
    }
}
