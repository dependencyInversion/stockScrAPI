pub struct Price {
    price: f32
}

impl Price {
    pub fn new() -> Price {
        Price { price: 0.0 }
    }

    pub fn from_string_slice(slice: &str) -> Price {
        Price { price: slice.parse::<f32>().unwrap() }
    }

    pub fn set_price(&mut self, input: &f32) {
        self.price  = input.clone();
    }

    pub fn get_price(&self) -> f32 { self.price.clone() }
}

#[cfg(test)]
mod price_test {
    use super::Price;

    #[test]
    pub fn set_price_expect_to_be_equal_to_input() {
        let i: f32 = 77.55;
        let mut p = Price::new();

        p.set_price(&i);

        assert_eq!(p.get_price(), i)
    }

    #[test]
    pub fn get_price_expect_to_be_equal_to_input() {
        let i = 77.9;
        let mut p = Price::new();

        p.set_price(&i);

        assert_eq!(p.get_price(), i)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_equal_to_input() {
        let i = "116.85";
        let p = Price::from_string_slice(i);

        assert_eq!(p.get_price(), 116.85)
    }
}