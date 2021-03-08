use short_scale_resolver::ShortScaleResolver;

pub struct AverageVolume {
    average_volume: i64
}

impl AverageVolume {
    pub fn new() -> AverageVolume {
        AverageVolume { average_volume: 0 }
    }

    pub fn from_string_slice(slice: &str) -> AverageVolume {
        AverageVolume { average_volume: ShortScaleResolver::from_string_slice(slice) }
    }

    pub fn set_average_volume(&mut self, input: i64) {
        self.average_volume  = input;
    }

    pub fn get_average_volume(&self) -> i64 { self.average_volume.clone() }
}

#[cfg(test)]
mod average_volume_test {
    use super::AverageVolume;

    #[test]
    pub fn from_string_slice_expect_result_to_be_eqaul_to_the_expected_value() {
        let input = "10M";
        let ten_million = 10 * i32::pow(10, 6) as i64;
        let v = AverageVolume::from_string_slice(input);

        assert_eq!(v.get_average_volume(), ten_million)
    }
}