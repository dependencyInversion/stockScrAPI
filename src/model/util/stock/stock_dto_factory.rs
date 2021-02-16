use crate::stock_dto::stock_dto;
use crate::short_scale_resolver::short_scale_resolver;
use crate::symbol::Symbol;
use crate::name::Name;

pub struct stock_dto_factory { }

impl stock_dto_factory {
    pub fn new() -> stock_dto_factory {
        stock_dto_factory{ }
    }

    pub fn from_string_slice_vector(&self, raw_stock_data: &Vec<&str>, resolver: &short_scale_resolver) -> stock_dto {
        
        let symbol = Symbol::from_string_slice(raw_stock_data[0]).get_symbol();
        let name = Name::from_string_slice(raw_stock_data[1]).get_name();
        let price = raw_stock_data[2].parse::<f32>().unwrap();
        let change = raw_stock_data[3].parse::<f32>().unwrap();
        let change_in_percentage = raw_stock_data[4].parse::<f32>().unwrap();
        let volume = resolver.from_string_slice(raw_stock_data[5]);
        let average_volume = resolver.from_string_slice(raw_stock_data[6]);
        let market_cap = resolver.from_string_slice(raw_stock_data[7]);
        let price_earnings_ratio = raw_stock_data[8].parse::<f32>().unwrap();

        stock_dto {
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