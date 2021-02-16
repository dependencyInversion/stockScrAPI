use crate::symbol::Symbol;
use crate::name::Name;

pub struct stock {
    pub symbol: Symbol,
    pub name: Name,
    pub price: f32,
    pub change: f32,
    pub change_in_percentage: f32,
    pub volume: i64,
    pub average_volume: i64,
    pub market_cap: i64,
    pub price_earnings_ratio: f32,
}