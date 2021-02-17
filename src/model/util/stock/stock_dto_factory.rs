use price::Price;

use crate::{price, stock::Stock};
use crate::short_scale_resolver::short_scale_resolver;
use crate::symbol::Symbol;
use crate::name::Name;
use crate::change::Change;
use crate::change_in_percentage::ChangeInPercentage;

pub struct stock_dto_factory { }

impl stock_dto_factory {
    pub fn new() -> stock_dto_factory {
        stock_dto_factory{ }
    }

    pub fn from_string_slice_vector(&self, raw_stock_data: &Vec<&str>, resolver: &short_scale_resolver) -> Stock {
        
        let symbol = Symbol::from_string_slice(raw_stock_data[0]);
        let name = Name::from_string_slice(raw_stock_data[1]);
        let price = Price::from_string_slice(raw_stock_data[2]);
        let change = Change::from_string_slice(raw_stock_data[3]);
        let change_in_percentage = ChangeInPercentage::from_string_slice_with_trailing_sign(raw_stock_data[4]);
        let volume = resolver.from_string_slice(raw_stock_data[5]);
        let average_volume = resolver.from_string_slice(raw_stock_data[6]);
        let market_cap = resolver.from_string_slice(raw_stock_data[7]);
        let price_earnings_ratio = raw_stock_data[8].parse::<f32>().unwrap();

        Stock {
            symbol,
            name,
            price,
            change,
            change_in_percentage, // cut off percent sign
            volume,
            average_volume,
            market_cap,
            price_earnings_ratio, // expect 'N/A'
        }
    }
}