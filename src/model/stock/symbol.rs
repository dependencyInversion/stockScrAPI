use std::str::FromStr;

pub struct Symbol {
    symbol: String
}

impl Symbol {
    pub fn new() -> Symbol {
        Symbol { symbol: String::new() }
    }

    pub fn from_string_slice(slice: &str) -> Symbol {
        Symbol { symbol: String::from_str(slice).unwrap() }
    }

    pub fn set_symbol(&mut self, input: &str) {
        self.symbol  = String::from_str(input).unwrap();
    }

    pub fn get_symbol(&self) -> String { self.symbol.clone() }
}

#[cfg(test)]
mod symbol_test {
    use super::Symbol;

    #[test]
    pub fn set_symbol_expect_to_be_equal_to_input() {
        let i = String::from("RLLCF");
        let mut s = Symbol::new();

        s.set_symbol(&i);

        assert_eq!(s.get_symbol(), i)
    }

    #[test]
    pub fn get_symbol_expect_to_be_equal_to_input() {
        let i = String::from("CMGR");
        let mut s = Symbol::new();

        s.set_symbol(&i);

        assert_eq!(s.get_symbol(), i)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_equal_to_input() {
        let i = "CMGR";
        let s = Symbol::from_string_slice(i);

        assert_eq!(s.get_symbol(), i)
    }
}