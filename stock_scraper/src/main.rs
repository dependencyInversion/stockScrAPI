extern crate reqwest;
extern crate scraper;

use reqwest::Response;
use scraper::{Html, Selector};

fn main() {
    println!("Hello, world!");
}

// Basic HTTP handling -> put it in a class later on
async fn scrape_from_url(url: &str) -> Response {
    let response: Response = reqwest::get(url).await.unwrap(); // never use .unwrap() for other than development purposes
    assert!(response.status().is_success()); // log other status codes

    response
}

async fn extract_body(response: Response) ->  String {
    let body = response.text().await.unwrap(); // why is everything async?

    body
}