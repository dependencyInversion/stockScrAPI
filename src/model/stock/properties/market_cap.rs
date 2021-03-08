use short_scale_resolver::ShortScaleResolver;

pub struct MarketCap {
    market_cap: i64
}

impl MarketCap {
    pub fn new() -> MarketCap {
        MarketCap { market_cap: 0 }
    }

    pub fn from_string_slice(slice: &str) -> MarketCap {
        MarketCap { market_cap: ShortScaleResolver::from_string_slice(slice) }
    }

    pub fn set_market_cap(&mut self, input: i64) {
        self.market_cap  = input;
    }

    pub fn get_market_cap(&self) -> i64 { self.market_cap.clone() }
}

#[cfg(test)]
mod market_cap_test {
    use super::MarketCap;

    #[test]
    pub fn from_string_slice_expect_result_to_be_eqaul_to_the_expected_value() {
        let input = "10M";
        let ten_million = 10 * i32::pow(10, 6) as i64;
        let m = MarketCap::from_string_slice(input);

        assert_eq!(m.get_market_cap(), ten_million)
    }
}