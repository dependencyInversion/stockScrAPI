pub struct stock_dto {
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