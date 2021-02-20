#[path = "../../stock/stock.rs"] mod stock;

use stock::{ Stock, AverageVolume, ChangeInPercentage, Change, MarketCap, Name, Price, Volume, Symbol };
use crate::short_scale_resolver::ShortScaleResolver;

pub struct stock_dto_factory { }

impl stock_dto_factory {
    pub fn new() -> stock_dto_factory {
        stock_dto_factory{ }
    }

    pub fn from_string_slice_vector(&self, raw_stock_data: &Vec<&str>, resolver: &ShortScaleResolver) -> Stock {
        
        let symbol = Symbol::from_string_slice(raw_stock_data[0]);
        let name = Name::from_string_slice(raw_stock_data[1]);
        let price = Price::from_string_slice(raw_stock_data[2]);
        let change = Change::from_string_slice(raw_stock_data[3]);
        let change_in_percentage = ChangeInPercentage::from_string_slice_with_trailing_sign(raw_stock_data[4]);
        let volume = Volume::from_string_slice(raw_stock_data[5]);
        let average_volume = AverageVolume::from_string_slice(raw_stock_data[6]);
        let market_cap = MarketCap::from_string_slice(raw_stock_data[7]);
        let price_earnings_ratio = raw_stock_data[8].parse::<f32>().unwrap();

        Stock {
            symbol,
            name,
            price,
            change,
            change_in_percentage,
            volume,
            average_volume,
            market_cap,
            price_earnings_ratio, // expect 'N/A'
        }
    }
}