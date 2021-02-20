use std::str::FromStr;

pub struct Name {
    name: String
}

impl Name {
    pub fn new() -> Name {
        Name { name: String::new() }
    }

    pub fn from_string_slice(slice: &str) -> Name {
        Name { name: String::from_str(slice).unwrap() }
    }

    pub fn set_name(&mut self, input: &str) {
        self.name  = String::from_str(input).unwrap();
    }

    pub fn get_name(&self) -> String { self.name.clone() }
}

#[cfg(test)]
mod name_test {
    use super::Name;

    #[test]
    pub fn set_name_expect_to_be_equal_to_input() {
        let i = String::from("Rolls-Royce Holdings plc");
        let mut n = Name::new();

        n.set_name(&i);

        assert_eq!(n.get_name(), i)
    }

    #[test]
    pub fn get_name_expect_to_be_equal_to_input() {
        let i = String::from("Clubhouse Media Group Inc.");
        let mut n = Name::new();

        n.set_name(&i);

        assert_eq!(n.get_name(), i)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_equal_to_input() {
        let i = "Clubhouse Media Group Inc.";
        let n = Name::from_string_slice(i);

        assert_eq!(n.get_name(), i)
    }
}