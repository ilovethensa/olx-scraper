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
        .version("0.1")
        .author("TheHolyTachanka")
        .subcommand(
            Command::new("search")
                .about("Searches OLX for products within a price range.")
                .arg(Arg::new("query").short('q').help("The search query").required(true))
                .arg(Arg::new("min_price").short('m').help("The minimum price").required(false))
                .arg(Arg::new("max_price").short('x').help("The maximum price").required(false)),
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
            let search_query = search_matches.get_one::<String>("query").unwrap().to_string();
            let min_price = search_matches.get_one::<String>("min_price").unwrap().to_string();
            let max_price = search_matches.get_one::<String>("max_price").unwrap().to_string();

            // Call your search function with the provided arguments
            let items = search::new(search_query, Some(min_price), Some(max_price), 5);

            println!("{items:#?}");
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
