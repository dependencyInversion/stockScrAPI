use short_scale_resolver::ShortScaleResolver;

pub struct Volume {
    volume: i64
}

impl Volume {
    pub fn new() -> Volume {
        Volume { volume: 0 }
    }

    pub fn from_string_slice(slice: &str) -> Volume {
        Volume { volume: ShortScaleResolver::from_string_slice(slice) }
    }

    pub fn set_volume(&mut self, input: i64) {
        self.volume  = input;
    }

    pub fn get_volume(&self) -> i64 { self.volume.clone() }
}

#[cfg(test)]
mod volume_test {
    use super::Volume;

    #[test]
    pub fn from_string_slice_expect_result_to_be_eqaul_to_the_expected_value() {
        let input = "10M";
        let ten_million = 10 * i32::pow(10, 6) as i64;
        let v = Volume::from_string_slice(input);

        assert_eq!(v.get_volume(), ten_million)
    }
}