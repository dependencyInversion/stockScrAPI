// Resource: https://en.wikipedia.org/wiki/Names_of_large_numbers

pub struct  short_scale_of_large_numbers {}

impl short_scale_of_large_numbers {
    
    fn get_value(indicator: char) -> i32 {
        match indicator {
            'M' => i32::pow(10, 6),
            'B' => i32::pow(10, 9),
            _ => panic!("'{}' can't be resolved in struct 'short_scale_of_large_numbers'", indicator),
        }
    }

}

// Add tests

#[cfg(test)]
mod short_scale_of_large_numbers_test {
    use super::short_scale_of_large_numbers;


    #[test]
    fn expect_output_to_be_one_million() {
        let value: i32 = short_scale_of_large_numbers::get_value('M');

        assert_eq!(value, 1000000);
    }

    #[test]
    fn expect_output_to_be_one_billion() {
        let value: i32 = short_scale_of_large_numbers::get_value('B');

        assert_eq!(value, 1000000000);
    }

    #[test]
    #[should_panic]
    fn expect_to_panic() {
        short_scale_of_large_numbers::get_value('T');
    }
}
