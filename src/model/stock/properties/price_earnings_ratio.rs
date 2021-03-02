pub struct PriceEarningsRatio {
    price_earnings_ratio: f32
}

impl PriceEarningsRatio {
    pub fn new() -> PriceEarningsRatio {
        PriceEarningsRatio { price_earnings_ratio: 0.0 }
    }

    pub fn from_string_slice(slice: &str) -> Option<PriceEarningsRatio> {
        
        if slice == "N/A" {
            return None;
        }

        Some(PriceEarningsRatio { price_earnings_ratio: slice.parse::<f32>().unwrap() })
    }

    pub fn set_price_earnings_ratio(&mut self, input: &f32) {
        self.price_earnings_ratio  = input.clone();
    }

    pub fn get_price_earnings_ratio(&self) -> f32 { self.price_earnings_ratio.clone() }
}

#[cfg(test)]
mod price_earnings_ratio_test {
    use super::PriceEarningsRatio;

    #[test]
    pub fn set_price_earnings_ratio_expect_to_be_equal_to_input() {
        let i: f32 = 32.89;
        let mut p = PriceEarningsRatio::new();

        p.set_price_earnings_ratio(&i);

        assert_eq!(p.get_price_earnings_ratio(), i)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_equal_to_input() {
        let i = "32.89";
        let p = PriceEarningsRatio::from_string_slice(i).unwrap();

        assert_eq!(p.get_price_earnings_ratio(), 32.89)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_none() {
        let i = "N/A";
        let p = PriceEarningsRatio::from_string_slice(i);

        assert!(p.is_none())
    }
}