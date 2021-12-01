use crate::util::file_reader::to_string_vector;

mod sonar_sweep_reader;

pub fn run_day_1() {
    let file_input = to_string_vector("inputs/day_1.txt").unwrap();

    let day_1_part_1_result = sonar_sweep_reader::get_number_of_increases(&file_input);
    let day_1_part_2_result = sonar_sweep_reader::get_number_of_three_sum_increases(&file_input);

    println!("Day 1 Part 1: {}", day_1_part_1_result);
    println!("Day 1 Part 2: {}", day_1_part_2_result);
}
