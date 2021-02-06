extern crate reqwest;
extern crate scraper;

use core::panic;

use reqwest::{Response, StatusCode, header::WARNING};
use scraper::{ElementRef, Html, Selector};
use log::{warn};

fn main() {
    // ToDo: Creater configuartion and configuration factory 
    let url: String = String::from("https://finance.yahoo.com/gainers?guccounter=1&guce_referrer=aHR0cHM6Ly9kdWNrZHVja2dvLmNvbS8&guce_referrer_sig=AQAAAAGs82DKSARjzO4JYBZn9fA-74sy2YUszrDA5-ArFjLTG2nyw1OAVXFTCHyaPvKZTqg_D5heNJKW3wvFuHQM8x18UvLYFOaw49eRDNtEL13B21htLk5BZBn8t2ZVmlCVkYvIJjrniE9p2idqscXY8SOWvIT9jbQR5dIocvSushLm");
    let selector: String = String::from(".simpTblRow");


    let handler = HttpHandler{ url }; // find a better matching name
    
    let mut res:Response = handler.get(); // add paramter
    if handler.is_response_ok(&res) {
        let body = handler.extract_body(&mut res);
        let doc = handler.parse_body_as_html(&body);
        handler.select(&doc, &selector);
    }
}


struct HttpHandler {
    url: String,
}

impl HttpHandler {
    
    fn get (&self) -> Response{

        let response: Response = reqwest::get(&self.url).unwrap(); // use match statement here

        response
    }

    fn is_response_ok (&self, response: &Response) -> bool {

        if !response.status().is_success() {
            warn!("status code: {}", response.status());
            panic!(); // find a better way to handle status codes
        }

        true
    }

    fn extract_body(&self, response: &mut Response) -> String {
        
        match response.text() {
            Ok(body) => body,
            
            Err(e) => {
                warn!("{}", e.to_string());
                panic!(); // find a better way to handle unwrapping in general
            },
        }
    }

    fn parse_body_as_html(&self, html_body: &String) -> Html { // different domain
        Html::parse_document(&html_body)
    }

    fn select(&self, html: &Html, css_selector: &String) { // different domain
        let selector = Selector::parse(css_selector).unwrap(); // match this later on

        for element in html.select(&selector) {
            let content = element.text().collect::<Vec<_>>(); // ToDo: Vec<&str> to DTO

            println!("{:?}", content); // what is ':?' ?
        }
    }
}