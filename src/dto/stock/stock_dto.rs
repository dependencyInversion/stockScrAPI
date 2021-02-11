pub struct stock_dto {
    symbol: String,
    name: String,
    price: f32,
    change: f32,
    change_in_percentage: f32,
    volume: i64,
    market_cap: i64,
    pe_ratio_ttm: f32,
}

impl stock_dto {
    pub fn new ( // move this into a factory; input (vec<vec<str>>); output (list<stock_dto>)

        symbol: &str,
        name: &str,
        price: &str,
        change: &str,
        change_in_percentage: &str,
        volume: &str,
        market_cap: &str,
        pe_ratio_ttm: &str,

    ) -> stock_dto {

        let symbol = String::from(symbol);
        let name = String::from(name);
        let price = price.parse::<f32>().unwrap();
        let change = change.parse::<f32>().unwrap();
        let change_in_percentage = change_in_percentage.parse::<f32>().unwrap();
        let volume = volume.parse::<i64>().unwrap();
        let market_cap = market_cap.parse::<i64>().unwrap();
        let pe_ratio_ttm = pe_ratio_ttm.parse::<f32>().unwrap();

        stock_dto {
            symbol,
            name,
            price,
            change,
            change_in_percentage,
            volume,
            market_cap,
            pe_ratio_ttm,
        }
    }
}