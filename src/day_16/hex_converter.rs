pub fn hex_str_to_binary_string(input: &str) -> String {
    input
        .chars()
        .map(|hex_char| hex_char_to_binary_string(hex_char))
        .collect()
}

fn hex_char_to_binary_string(hex_char: char) -> String {
    let binary_string = match hex_char {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Could not convert hex char: {}", hex_char),
    };

    binary_string.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 3] = ["D2FE28", "38006F45291200", "EE00D40C823060"];

    #[test]
    fn test_hex_str_to_binary_string() {
        let expected = vec![
            String::from("110100101111111000101000"),
            String::from("00111000000000000110111101000101001010010001001000000000"),
            String::from("11101110000000001101010000001100100000100011000001100000"),
        ];

        let result: Vec<String> = TEST_DATA
            .iter()
            .map(|hex_str| hex_str_to_binary_string(hex_str))
            .collect();

        assert_eq!(result, expected);
    }
}
