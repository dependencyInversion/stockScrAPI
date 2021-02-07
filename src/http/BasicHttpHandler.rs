extern crate reqwest;
extern crate scraper;

use core::panic;

use reqwest::{Response, StatusCode, header::WARNING};
use scraper::{ElementRef, Html, Selector};
use log::{warn};

pub struct BasicHttpHandler {
    pub url: String,
}

impl BasicHttpHandler {

    pub fn get (&self) -> Response{

        let response: Response = reqwest::get(&self.url).unwrap(); // use match statement here

        response
    }

    pub fn is_response_ok (&self, response: &Response) -> bool {

        if !response.status().is_success() {
            warn!("status code: {}", response.status());
            panic!(); // find a better way to handle status codes
        }

        true
    }

    pub fn extract_body(&self, response: &mut Response) -> String {

        match response.text() {
            Ok(body) => body,

            Err(e) => {
                warn!("{}", e.to_string());
                panic!(); // find a better way to handle unwrapping in general
            },
        }
    }

    pub fn parse_body_as_html(&self, html_body: &String) -> Html { // different domain
        Html::parse_document(&html_body)
    }

    pub fn select(&self, html: &Html, css_selector: &String) { // different domain
        let selector = Selector::parse(css_selector).unwrap(); // match this later on

        for element in html.select(&selector) {
            let content = element.text().collect::<Vec<_>>(); // ToDo: Vec<&str> to DTO

            println!("{:?}", content); // what is ':?' ?
        }
    }
}