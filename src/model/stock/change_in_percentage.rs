use crate::change;

pub struct ChangeInPercentage {
    change_in_percentage: f32
}

impl ChangeInPercentage {
    pub fn new() -> ChangeInPercentage {
        ChangeInPercentage { change_in_percentage: 0.0 }
    }

    pub fn from_string_slice_with_trailing_sign(slice: &str) -> ChangeInPercentage {
        let mut c: f32 = 0.0;
        
        {
            let mut input: String = String::from(slice);
            input.pop();
    
            c = input.parse::<f32>().unwrap()
        }

        ChangeInPercentage { change_in_percentage: c }
    }

    pub fn set_change_in_percentage_(&mut self, input: &f32) {
        self.change_in_percentage  = input.clone();
    }

    pub fn get_change_in_percentage(&self) -> f32 { self.change_in_percentage.clone() }
}

#[cfg(test)]
mod change_test {
    use super::ChangeInPercentage;

    #[test]
    pub fn set_change_in_percentage_expect_to_be_equal_to_input() {
        let i: f32 = 67.49;
        let mut c = ChangeInPercentage::new();

        c.set_change_in_percentage_(&i);

        assert_eq!(c.get_change_in_percentage(), i)
    }

    #[test]
    pub fn get_change_in_percentage_expect_to_be_equal_to_input() {
        let i = 77.9;
        let mut c = ChangeInPercentage::new();

        c.set_change_in_percentage_(&i);

        assert_eq!(c.get_change_in_percentage(), i)
    }

    #[test]
    pub fn from_string_slice_expect_to_be_equal_to_input() {
        let i = "+116.85%";
        let c = ChangeInPercentage::from_string_slice_with_trailing_sign(i);

        assert_eq!(c.get_change_in_percentage(), 116.85)
    }
}