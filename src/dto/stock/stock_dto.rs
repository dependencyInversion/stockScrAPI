#[path ="../../model/stock/stock.rs"] mod stock;

use std::fmt;
use stock::{ Stock };
use crate::{change_in_percentage, volume};

pub struct StockDto {
    pub symbol: String,
    pub name: String,
    pub price: f32,
    pub change: f32,
    pub change_in_percentage: f32,
    pub volume: i64,
    pub average_volume: i64,
    pub market_cap: i64,
    pub price_earnings_ratio: f32,
}

impl StockDto {
    pub fn from_stock(stock: Stock) -> StockDto {
        StockDto {
            symbol: stock.symbol.get_symbol(),
            name: stock.name.get_name(),
            price: stock.price.get_price(),
            change: stock.change.get_change(),
            change_in_percentage: stock.change_in_percentage.get_change_in_percentage(),
            volume: stock.volume.get_volume(),
            average_volume: stock.average_volume.get_average_volume(),
            market_cap: stock.market_cap.get_market_cap(),
            price_earnings_ratio: stock.price_earnings_ratio.get_price_earnings_ratio()
        }
    }
}

impl fmt::Debug for StockDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StockDto")
         .field("symbol", &self.symbol)
         .field("name", &self.name)
         .field("price", &self.price)
         .field("change", &self.change)
         .field("change_in_percentage", &self.change_in_percentage)
         .field("volume", &self.volume)
         .field("average_volume", &self.average_volume)
         .field("market_cap", &self.market_cap)
         .field("price_earnings_ratio", &self.price_earnings_ratio)
         .finish()
    }
}