enum Columns {
    Symbol,
    Name,
    Price,
    Change,
    ChangeInPercentage,
    Volume,
    AverageVolume,
    MarketCap,
    PriceEarningsRatio
}

pub struct stock_dto {
    pub symbol: String,
    name: String,
    price: f32,
    change: f32,
    change_in_percentage: f32,
    volume: i64,
    average_volume: i64,
    market_cap: i64,
    price_earnings_ratio: f32,
}

impl stock_dto {
    pub fn new (

        symbol: &str,
        name: &str,
        price: &str,
        change: &str,
        change_in_percentage: &str,
        volume: &str,
        average_volume: &str,
        market_cap: &str,
        price_earnings_ratio: &str,

    ) -> stock_dto {

        let symbol = String::from(symbol);
        let name = String::from(name);
        let price = price.parse::<f32>().unwrap();
        let change = change.parse::<f32>().unwrap();
        let change_in_percentage = change_in_percentage.parse::<f32>().unwrap();
        let volume = volume.parse::<i64>().unwrap();
        let average_volume = average_volume.parse::<i64>().unwrap();
        let market_cap = market_cap.parse::<i64>().unwrap();
        let price_earnings_ratio = price_earnings_ratio.parse::<f32>().unwrap();

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