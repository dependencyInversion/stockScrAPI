// Resource: https://en.wikipedia.org/wiki/Names_of_large_numbers

pub struct  large_number_resolver {}

impl large_number_resolver {
    
    fn get_value(indicator: char) -> i32 {
        match indicator {
            'M' => i32::pow(10, 6),
            'B' => i32::pow(10, 9),
            _ => panic!("'{}' can't be resolved in struct 'large_number_resolver'", indicator),
        }
    }

}

// Move to test directory later on

#[cfg(test)]
mod short_scale_of_large_numbers_test {
    use super::large_number_resolver;


    #[test]
    fn expect_output_to_be_one_million() {
        let value: i32 = large_number_resolver::get_value('M');

        assert_eq!(value, 1000000);
    }

    #[test]
    fn expect_output_to_be_one_billion() {
        let value: i32 = large_number_resolver::get_value('B');

        assert_eq!(value, 1000000000);
    }

    #[test]
    #[should_panic]
    fn expect_to_panic() {
        large_number_resolver::get_value('T');
    }
}
