# OLX Scraper

## Overview

OLX Scraper is a Rust library for extracting product information from OLX (www.olx.bg). It provides functionalities for searching products within a specified price range and retrieving details about individual items using their URLs. Additionally, an optional command-line tool is included for convenient usage.

## Features

- **Search Functionality**: Search for products on OLX based on a query and optional price range.
- **Item Details**: Retrieve detailed information about an item using its OLX URL.
- **Pagination Support**: The search function supports pagination, allowing users to retrieve results from multiple pages.
- **Error Handling**: The library gracefully handles errors during the search and item retrieval processes.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
olx-scraper = "0.1"
```
# Usage
## Library

```rust
use olx_scraper::search;

search::new("laptop", Some("elektronika/kompyutri/laptopi"), Some("500".to_string()), Some("1000".to_string()), Some("5"));
// Process the 'items' vector as needed
```

## Command-Line Tool

```bash
olx-scraper search -q <query> [-m <min_price>] [-x <max_price>] [-p <end_page>] [ -c <category>]
```
-    <query>: The search query.
-    <min_price> (optional): The minimum price filter.
-    <max_price> (optional): The maximum price filter.
-    <end_page> (optional): The page to end the search on
-    <category> (optional): The category of the item you want to search for

```bash
olx-scraper get --url <item_url>
```
-    <item_url>: The URL of the item on OLX.

## Examples
Search for Laptops Priced between $500 and $1000 in the category laptops

```bash
olx-scraper search -q laptop -m 500 -x 1000 -c elektronika/kompyutri/laptopi
```

Get Details of a Specific Item

```bash
olx-scraper get --url https://www.olx.bg/item/example-item
```
