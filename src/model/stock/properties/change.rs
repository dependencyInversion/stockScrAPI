pub struct Change {
    change: f32
}

impl Change {
    pub fn new() -> Change {
        Change { change: 0.0 }
    }

    pub fn from_string_slice(slice: &str) -> Change {
        Change { change: slice.parse::<f32>().unwrap() }
    }

    pub fn set_change(&mut self, input: &f32) {
        self.change  = input.clone();
    }

    pub fn get_change(&self) -> f32 { self.change.clone() }
}

#[cfg(test)]
mod change_test {
    use super::Change;

    #[test]
    pub fn set_change_expect_to_be_equal_to_input() {
        let i: f32 = 77.55;
        let mut c = Change::new();

        c.set_change(&i);

        assert_eq!(c.get_change(), i)
    }

    #[test]
    pub fn get_change_expect_to_be_equal_to_input() {
        let i = 77.9;
        let mut c = Change::new();

        c.set_change(&i);

        assert_eq!(c.get_change(), i)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_equal_to_input() {
        let i = "116.85";
        let c = Change::from_string_slice(i);

        assert_eq!(c.get_change(), 116.85)
    }
}