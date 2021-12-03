mod binary_diagnostic;

use crate::util::file_reader::to_string_vector;

pub fn run_day_3() {
    let file_input = to_string_vector("inputs/day_3.txt").unwrap();

    let part_1_result = binary_diagnostic::get_power_consumption(&file_input);
    let part_2_result = binary_diagnostic::get_life_support_rating(&file_input);

    println!("Day 3 Part 1: {}", part_1_result);
    println!("Day 3 Part 2: {}", part_2_result);
}
