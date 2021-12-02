mod pilot_computer;

use crate::util::file_reader::to_string_vector;

pub fn run_day_2() {
    let file_input = to_string_vector("inputs/day_2.txt").unwrap();

    let submarine_location = pilot_computer::get_final_location(&file_input);

    let part_1_result = submarine_location.x.abs() * submarine_location.y.abs();
    let part_2_result = submarine_location.x.abs() * submarine_location.z.abs();

    println!("Day 2 Part 1: {}", part_1_result);
    println!("Day 2 Part 2: {}", part_2_result);
}
