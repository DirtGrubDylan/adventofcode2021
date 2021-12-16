pub fn str_slice_to_string_vector(input: &[&str]) -> Vec<String> {
    input.iter().map(|s| s.to_string()).collect()
}
