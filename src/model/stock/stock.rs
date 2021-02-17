use crate::symbol::Symbol;
use crate::name::Name;
use crate::price::Price;
use crate::change::Change;
use crate::change_in_percentage::ChangeInPercentage;

pub struct Stock {
    pub symbol: Symbol,
    pub name: Name,
    pub price: Price,
    pub change: Change,
    pub change_in_percentage: ChangeInPercentage,
    pub volume: i64,
    pub average_volume: i64,
    pub market_cap: i64,
    pub price_earnings_ratio: f32,
}