extern crate reqwest;

#[path = "http/basic_http_handler.rs"] mod basic_http_handler;
#[path = "dto/stock_dto.rs"] mod stock_dto;

use reqwest::{Response};

fn main() {
    // ToDo: Creater configuartion and configuration factory 
    let url: String = String::from("https://finance.yahoo.com/gainers?guccounter=1&guce_referrer=aHR0cHM6Ly9kdWNrZHVja2dvLmNvbS8&guce_referrer_sig=AQAAAAGs82DKSARjzO4JYBZn9fA-74sy2YUszrDA5-ArFjLTG2nyw1OAVXFTCHyaPvKZTqg_D5heNJKW3wvFuHQM8x18UvLYFOaw49eRDNtEL13B21htLk5BZBn8t2ZVmlCVkYvIJjrniE9p2idqscXY8SOWvIT9jbQR5dIocvSushLm");
    let selector: String = String::from(".simpTblRow");


    let handler = basic_http_handler::BasicHttpHandler{ url }; // find a better matching name
    
    let mut res:Response = handler.get(); // add paramter
    if handler.is_response_ok(&res) {
        let body = handler.extract_body(&mut res);
        let doc = handler.parse_body_as_html(&body);
        handler.select(&doc, &selector);
    }

    // let stock: stock_dto::stock_dto = stock_dto::stock_dto::new(symbol: &str, name: &str, price: &str, change: &str, change_in_percentage: &str, volume: &str, market_cap: &str, pe_ratio_ttm: &str)
}