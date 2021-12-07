#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrabSubmarine {
    x_position: i32,
}

impl CrabSubmarine {
    pub fn new(x_position: i32) -> CrabSubmarine {
        CrabSubmarine { x_position }
    }

    pub fn get_x_position(&self) -> i32 {
        self.x_position
    }

    pub fn linear_distance_to(&self, other_x: i32) -> i32 {
        (other_x - self.x_position).abs()
    }

    pub fn guassian_distance_to(&self, other_x: i32) -> i32 {
        let linear_distance = self.linear_distance_to(other_x);

        linear_distance * (linear_distance + 1) / 2
    }
}

pub fn get_crab_submarines(input: &str) -> Vec<CrabSubmarine> {
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| CrabSubmarine::from(s))
        .collect()
}

pub fn minimum_fuel_to_align_v1(crab_submarines: &[CrabSubmarine]) -> i32 {
    let mut submarines_sorted = crab_submarines.to_vec();

    submarines_sorted.sort();

    let median_index = submarines_sorted.len() / 2;

    let median_value = submarines_sorted
        .get(median_index)
        .expect(&format!(
            "Could not get median index of: {:?}",
            submarines_sorted
        ))
        .get_x_position();

    submarines_sorted
        .iter()
        .fold(0, |acc, sub| acc + sub.linear_distance_to(median_value))
}

pub fn minimum_fuel_to_align_v2(crab_submarines: &[CrabSubmarine]) -> i32 {
    let mut submarines_sorted = crab_submarines.to_vec();

    submarines_sorted.sort();

    let mut start = 0;
    let mut end = submarines_sorted.len() as i32;

    let mut result = 0;

    while start != end {
        let median = start + (end - start) / 2;

        let left_median_value = crab_submarines
            .iter()
            .fold(0, |acc, sub| acc + sub.guassian_distance_to(median - 1));

        let median_value = crab_submarines
            .iter()
            .fold(0, |acc, sub| acc + sub.guassian_distance_to(median));

        let right_median_value = crab_submarines
            .iter()
            .fold(0, |acc, sub| acc + sub.guassian_distance_to(median + 1));

        if left_median_value < median_value {
            end = median - 1;
        } else if right_median_value < median_value {
            start = median + 1;
        } else {
            result = median_value;

            break;
        }
    }

    result
}

impl From<&str> for CrabSubmarine {
    fn from(input: &str) -> CrabSubmarine {
        let x_position = input
            .parse()
            .expect(&format!("Could not parse into i32: {}", input));

        CrabSubmarine::new(x_position)
    }
}

impl From<&String> for CrabSubmarine {
    fn from(input: &String) -> CrabSubmarine {
        let x_position = input
            .parse()
            .expect(&format!("Could not parse into i32: {}", input));

        CrabSubmarine::new(x_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_crab_submarine_from_str() {
        let expected = CrabSubmarine::new(16);

        let result = CrabSubmarine::from("16");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_crab_submarine_linear_distance() {
        let other_x = 5;

        let crab_submarine_1 = CrabSubmarine::new(16);
        let crab_submarine_2 = CrabSubmarine::new(0);

        let expected_1 = 11;
        let expected_2 = 5;

        let result_1 = crab_submarine_1.linear_distance_to(other_x);
        let result_2 = crab_submarine_2.linear_distance_to(other_x);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_crab_submarine_guassian_distance() {
        let other_x = 5;

        let crab_submarine_1 = CrabSubmarine::new(16);
        let crab_submarine_2 = CrabSubmarine::new(0);

        let expected_1 = 66;
        let expected_2 = 15;

        let result_1 = crab_submarine_1.guassian_distance_to(other_x);
        let result_2 = crab_submarine_2.guassian_distance_to(other_x);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_crab_submarines_sorted() {
        let mut submarines = vec![
            CrabSubmarine::new(16),
            CrabSubmarine::new(1),
            CrabSubmarine::new(2),
            CrabSubmarine::new(0),
            CrabSubmarine::new(4),
            CrabSubmarine::new(2),
            CrabSubmarine::new(7),
            CrabSubmarine::new(1),
            CrabSubmarine::new(2),
            CrabSubmarine::new(14),
        ];

        let expected = vec![
            CrabSubmarine::new(0),
            CrabSubmarine::new(1),
            CrabSubmarine::new(1),
            CrabSubmarine::new(2),
            CrabSubmarine::new(2),
            CrabSubmarine::new(2),
            CrabSubmarine::new(4),
            CrabSubmarine::new(7),
            CrabSubmarine::new(14),
            CrabSubmarine::new(16),
        ];

        submarines.sort();

        assert_eq!(submarines, expected);
    }

    #[test]
    fn test_get_crab_submarines() {
        let expected = vec![
            CrabSubmarine::new(16),
            CrabSubmarine::new(1),
            CrabSubmarine::new(2),
            CrabSubmarine::new(0),
            CrabSubmarine::new(4),
            CrabSubmarine::new(2),
            CrabSubmarine::new(7),
            CrabSubmarine::new(1),
            CrabSubmarine::new(2),
            CrabSubmarine::new(14),
        ];

        let result = get_crab_submarines(TEST_DATA);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_minimum_fuel_to_align_v1() {
        let submarines = get_crab_submarines(TEST_DATA);

        let expected = 37;

        let result = minimum_fuel_to_align_v1(&submarines);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_minimum_fuel_to_align_v2() {
        let submarines = get_crab_submarines(TEST_DATA);

        let expected = 168;

        let result = minimum_fuel_to_align_v2(&submarines);

        assert_eq!(result, expected);
    }
}
