#[path = "./properties/change_in_percentage.rs"] mod change_in_percentage;
#[path = "./properties/change.rs"] mod change;
#[path = "./properties/market_cap.rs"] mod market_cap;
#[path = "./properties/name.rs"] mod name;
#[path = "./properties/price.rs"] mod price;
#[path = "./properties/symbol.rs"] mod symbol;
#[path = "./properties/volume.rs"] mod volume;
#[path = "./properties/average_volume.rs"] mod average_volume;
#[path = "./properties/price_earnings_ratio.rs"] mod price_earnings_ratio;

pub use change_in_percentage::ChangeInPercentage;
pub use change::Change;
pub use market_cap::MarketCap;
pub use name::Name;
pub use price::Price;
pub use volume::Volume;
pub use symbol::Symbol;
pub use average_volume::AverageVolume;
pub use price_earnings_ratio::PriceEarningsRatio;

pub struct Stock {
    pub symbol: Symbol,
    pub name: Name,
    pub price: Price,
    pub change: Change,
    pub change_in_percentage: ChangeInPercentage,
    pub volume: Volume,
    pub average_volume: AverageVolume,
    pub market_cap: MarketCap,
    pub price_earnings_ratio: PriceEarningsRatio,
}