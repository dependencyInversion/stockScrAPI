use std::{i64};
use regex::Regex;
use log::{warn};

// Resource: https://en.wikipedia.org/wiki/Names_of_large_numbers

/* Todo:
 *       1. Cover all test cases
 *            -> M/B to 10^n
 *            -> ...
 *       2. Resolv all calls of `.unwrap()`
 *       3. Find better matching variable an method names
 *       4. Improve names of tests
*/

pub struct  short_scale_resolver {}

impl short_scale_resolver {
    
    pub fn short_scale_to_i64(&self, indicator: &char) -> i64 {
        match indicator {
            'M' => i32::pow(10, 6) as i64,
            'B' => i32::pow(10, 9) as i64,
            _ => {
                warn!("{} could not be resolved as shot scale of large numbers!", indicator);
                1 as i64
            },
        }
    }

    fn extract_indicator(&self, input: &str) -> char {
        
        let re = Regex::new(r"([MB]$)").unwrap();
        let mut indicator: &str = "";

        for caps in re.captures_iter(input).take(1) {
            indicator = caps.get(0).unwrap().as_str();
        }
        
        indicator.chars().rev().last().unwrap()
    }

    fn extract_multiplier(&self, input: &str) -> i64 {

        let re = Regex::new(r"(^(\d)+)").unwrap();
        let mut base: &str = "";

        for caps in re.captures_iter(input).take(1) {
            base = caps.get(0).unwrap().as_str();
        }

        base.parse::<i64>().unwrap()

    }

    fn has_indicator(&self, input: &str) -> bool {
        let re: Regex = Regex::new(r"([MB]$)").unwrap();
        
        re.is_match(input)
    }

    pub fn from_string_slice(&self, input: &str) -> i64 {
        
        let mut res: i64 = self.extract_multiplier(&input);
        
        if self.has_indicator(input) {
            let indicator = self.extract_indicator(&input);
            let factor = self.short_scale_to_i64(&indicator);

            res *= factor;
        }

        res
    }
}

// Move tests to separate directory later on

#[cfg(test)]
mod short_scale_resolver_test {
    use super::short_scale_resolver;

    #[test]
    fn expect_output_to_be_one_million() {
        let input = 'M';
        let resolver = short_scale_resolver{};

        assert_eq!(resolver.short_scale_to_i64(&input), 1000000);
    }

    #[test]
    fn expect_output_to_be_one_billion() {
        let input = 'B';
        let resolver = short_scale_resolver{};

        assert_eq!(resolver.short_scale_to_i64(&input), 1000000000);
    }

    #[test]
    fn expect_to_panic() {
        let invalid_input = 'T';
        let resolver = short_scale_resolver{};

        let result = resolver.short_scale_to_i64(&invalid_input);

        assert_eq!(result, 1)
    }

    #[test]
    fn extract_indicator_test_expect_to_return_M () {
        let resolver = short_scale_resolver{};
        let result = resolver.extract_indicator("10M");

        assert_eq!('M', result)
    }

    #[test]
    fn extract_indicator_test_expect_to_return_B () {
        let resolver = short_scale_resolver{};
        let result = resolver.extract_indicator("10B");

        assert_eq!('B', result)
    }

    #[test]
    fn t() {
        let resolver = short_scale_resolver{};
        resolver.from_string_slice("10M");
    }

    #[test]
    fn from_raw_expect_to_return_ten_million() {
        let input = "10M";
        let ten_million = 10 * i32::pow(10, 6) as i64;
        let resolver = short_scale_resolver{};

        assert_eq!(resolver.from_string_slice(input), ten_million);
    }

    #[test]
    fn from_raw_expect_to_return_ten() {
        let input = "10";
        let ten_million: i64 = 10;
        let resolver = short_scale_resolver{};

        assert_eq!(resolver.from_string_slice(input), ten_million);
    }
}
