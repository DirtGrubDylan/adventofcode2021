use std::fs;

pub fn to_string_vector(file_name: &str) -> Result<Vec<String>, String> {
    Ok(fs::read_to_string(file_name)
        .expect("File not found!")
        .trim_end()
        .split("\n")
        .map(|line| line.to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_vector() {
        let expected = vec![
            String::from("1721"),
            String::from("979"),
            String::from("366"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        ];

        let result = to_string_vector("test_inputs/day_1_part_1.txt").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_to_string_vector_error() {
        let _result = to_string_vector("test_inputs/day_100000000.txt");
    }
}
