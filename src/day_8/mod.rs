mod seven_segment_display;

use crate::util::file_reader::to_string_vector;

pub fn run_day_8() {
    let file_input = to_string_vector("inputs/day_8.txt").unwrap();

    let part_1_result = seven_segment_display::get_number_of_1_4_7_or_8_displays(&file_input);
    let part_2_result: u32 = seven_segment_display::get_displays_for(&file_input)
        .iter()
        .sum();

    println!("Day 8 Part 1: {}", part_1_result);
    println!("Day 8 Part 2: {}", part_2_result);
}
