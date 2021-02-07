extern crate reqwest;
extern crate scraper;

use core::panic;

use reqwest::{Response, StatusCode, header::WARNING};
use scraper::{ElementRef, Html, Selector};
use log::{warn};

#[path = "http/BasicHttpHandler.rs"] mod BasicHttpHandler;

fn main() {
    // ToDo: Creater configuartion and configuration factory 
    let url: String = String::from("https://finance.yahoo.com/gainers?guccounter=1&guce_referrer=aHR0cHM6Ly9kdWNrZHVja2dvLmNvbS8&guce_referrer_sig=AQAAAAGs82DKSARjzO4JYBZn9fA-74sy2YUszrDA5-ArFjLTG2nyw1OAVXFTCHyaPvKZTqg_D5heNJKW3wvFuHQM8x18UvLYFOaw49eRDNtEL13B21htLk5BZBn8t2ZVmlCVkYvIJjrniE9p2idqscXY8SOWvIT9jbQR5dIocvSushLm");
    let selector: String = String::from(".simpTblRow");


    let handler = BasicHttpHandler::BasicHttpHandler{ url }; // find a better matching name
    
    let mut res:Response = handler.get(); // add paramter
    if handler.is_response_ok(&res) {
        let body = handler.extract_body(&mut res);
        let doc = handler.parse_body_as_html(&body);
        handler.select(&doc, &selector);
    }
}