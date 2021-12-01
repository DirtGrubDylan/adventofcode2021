pub fn get_number_of_increases(depth_data: &[String]) -> u32 {
    let mut result = 0;

    let mut previous_depth_option: Option<u32> = None;

    for depth_str in depth_data {
        let current_depth = depth_str
            .parse::<u32>()
            .expect(&format!("Cannot parse depth: {}", depth_str));

        result += get_parity_optional_comparison(current_depth, previous_depth_option);

        previous_depth_option = Some(current_depth);
    }

    result
}

pub fn get_number_of_three_sum_increases(depth_data: &[String]) -> u32 {
    let mut result = 0;

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut sum_3 = 0;

    for (index, depth_str) in depth_data.iter().enumerate() {
        let current_depth = depth_str
            .parse::<u32>()
            .expect(&format!("Cannot parse depth: {}", depth_str));

        if index < 3 {
            sum_1 += current_depth;
        }

        if 1 <= index {
            sum_2 += current_depth;
        }

        if 2 <= index {
            sum_3 += current_depth;
        }

        if 3 <= index {
            result += get_parity_comparison(sum_2, sum_1);

            sum_1 = sum_2;
            sum_2 = sum_3;
            sum_3 = current_depth;
        }
    }

    result
}

fn get_parity_optional_comparison(current_number: u32, previous_number_option: Option<u32>) -> u32 {
    match previous_number_option {
        Some(previous_number) => get_parity_comparison(current_number, previous_number),
        None => 0,
    }
}

fn get_parity_comparison(current_number: u32, previous_number: u32) -> u32 {
    if previous_number < current_number {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 10] = [
        "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
    ];

    #[test]
    fn test_get_number_of_increases() {
        let input: Vec<String> = TEST_DATA.iter().map(|data| data.to_string()).collect();

        let expected = 7;
        let result = get_number_of_increases(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_number_of_three_sum_increases() {
        let input: Vec<String> = TEST_DATA.iter().map(|data| data.to_string()).collect();

        let expected = 5;
        let result = get_number_of_three_sum_increases(&input);

        assert_eq!(result, expected);
    }
}
