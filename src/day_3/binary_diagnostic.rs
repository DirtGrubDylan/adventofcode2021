use std::u32;

pub fn get_power_consumption(input: &[String]) -> u32 {
    let length_of_binary_string = input[0].len();

    let mut counter = vec![0; length_of_binary_string];

    for bit_string in input {
        for (index, bit_char) in bit_string.chars().enumerate() {
            let current_column_count = counter
                .get_mut(index)
                .expect(&format!("Can't get count at index: {}", index));

            match bit_char {
                '0' => *current_column_count -= 1,
                '1' => *current_column_count += 1,
                _ => panic!("Unknown bit {} in bit_string {}", bit_char, bit_string),
            };
        }
    }

    let gamma_rate = array_of_bits_to_decimal(&get_most_signifcant_bits(&counter));
    let epsilon_rate = array_of_bits_to_decimal(&get_least_signifcant_bits(&counter));

    gamma_rate * epsilon_rate
}

pub fn get_life_support_rating(input: &[String]) -> u32 {
    let length_of_binary_string = input[0].len();

    let mut oxygen_generator_values = input.to_vec();
    let mut c02_scrubber_values = input.to_vec();

    for column in 0..length_of_binary_string {
        if oxygen_generator_values.len() != 1 {
            let oxygen_most_signifcant_bit =
                get_most_signifcant_bit_at(&oxygen_generator_values, column);

            oxygen_generator_values = filter_for_number_with_bits_at(
                &oxygen_generator_values,
                oxygen_most_signifcant_bit,
                column,
            );
        }

        if c02_scrubber_values.len() != 1 {
            let c02_least_signifcant_bit =
                get_least_signifcant_bit_at(&c02_scrubber_values, column);

            c02_scrubber_values = filter_for_number_with_bits_at(
                &c02_scrubber_values,
                c02_least_signifcant_bit,
                column,
            );
        }
    }

    if oxygen_generator_values.len() != 1 {
        panic!("Too many oxygen values!: {:?}", oxygen_generator_values);
    } else if c02_scrubber_values.len() != 1 {
        panic!("Too many c02 values!: {:?}", c02_scrubber_values);
    }

    let oxygen_generator_rating_str = oxygen_generator_values.get(0).unwrap();
    let oxygen_generator_rating = u32::from_str_radix(oxygen_generator_rating_str, 2).unwrap();

    let c02_scrubber_rating_str = c02_scrubber_values.get(0).unwrap();
    let c02_scrubber_rating = u32::from_str_radix(c02_scrubber_rating_str, 2).unwrap();

    oxygen_generator_rating * c02_scrubber_rating
}

fn get_most_signifcant_bit_at(input: &[String], column: usize) -> u32 {
    let mut counter = 0;

    for bit_string in input {
        match bit_string.chars().nth(column) {
            Some('0') => counter -= 1,
            Some('1') => counter += 1,
            _ => panic!("Column {} too large for bit_string {}", column, bit_string),
        }
    }

    if counter < 0 {
        0
    } else {
        1
    }
}

fn get_least_signifcant_bit_at(input: &[String], column: usize) -> u32 {
    let most_signifcant_bit = get_most_signifcant_bit_at(input, column);

    match most_signifcant_bit {
        0 => 1,
        1 => 0,
        _ => panic!("Unknown bit: {}", most_signifcant_bit),
    }
}

fn filter_for_number_with_bits_at(input: &[String], bit: u32, column: usize) -> Vec<String> {
    input
        .iter()
        .filter(|binary| {
            let binary_char = binary
                .chars()
                .nth(column)
                .expect(&format!("Column too big: {}", column))
                .to_digit(10)
                .expect(&format!("Unknown char: {}", binary));

            binary_char == bit
        })
        .map(|s| s.to_string())
        .collect()
}

fn get_most_signifcant_bits(counted_bits: &[i32]) -> Vec<u32> {
    counted_bits
        .iter()
        .map(|&count| if count < 0 { 0 } else { 1 })
        .collect()
}

fn get_least_signifcant_bits(counted_bits: &[i32]) -> Vec<u32> {
    counted_bits
        .iter()
        .map(|&count| if count < 0 { 1 } else { 0 })
        .collect()
}

fn array_of_bits_to_decimal(bits: &[u32]) -> u32 {
    bits.iter()
        .fold(0, |running_sum, bit| (running_sum << 1) + bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    const COUNTED_BITS: [i32; 5] = [2, -3, 4, 2, -2];

    #[test]
    fn test_get_power_consumption() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = 198;

        let result = get_power_consumption(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_life_support_rating() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = 230;

        let result = get_life_support_rating(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_most_signifcant_bit_at() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected_1 = 1;
        let expected_2 = 0;

        let result_1 = get_most_signifcant_bit_at(&input, 2);
        let result_2 = get_most_signifcant_bit_at(&input, 4);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_least_signifcant_bit_at() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected_1 = 0;
        let expected_2 = 1;

        let result_1 = get_least_signifcant_bit_at(&input, 2);
        let result_2 = get_least_signifcant_bit_at(&input, 4);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_filter_for_number_with_bits_at() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected_1 = vec![
            "11110", "10110", "10111", "10101", "11100", "10000", "11001",
        ];
        let expected_2 = vec!["00100", "10101", "11100", "10000", "11001"];

        let result_1 = filter_for_number_with_bits_at(&input, 1, 0);
        let result_2 = filter_for_number_with_bits_at(&input, 0, 3);

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_most_signifcant_bits() {
        let expected = vec![1, 0, 1, 1, 0];

        let result = get_most_signifcant_bits(&COUNTED_BITS);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_least_signifcant_bits() {
        let expected = vec![0, 1, 0, 0, 1];

        let result = get_least_signifcant_bits(&COUNTED_BITS);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_of_bits_to_decimal() {
        let bits = vec![1, 0, 1, 1, 0];

        let expected = 22;

        let result = array_of_bits_to_decimal(&bits);

        assert_eq!(result, expected);
    }
}
