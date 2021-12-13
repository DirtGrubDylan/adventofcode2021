mod crab_submarine;

use crate::util::file_reader::to_string_vector;

pub fn run_day_7() {
    let file_input = to_string_vector("inputs/day_7.txt").unwrap();

    let crab_submarines = crab_submarine::get_crab_submarines(
        file_input.get(0).expect(&format!("File input empty!")),
    );

    let part_1_result = crab_submarine::minimum_fuel_to_align_v1(&crab_submarines);
    let part_2_result = crab_submarine::minimum_fuel_to_align_v2(&crab_submarines);

    println!("Day 7 Part 1: {}", part_1_result);
    println!("Day 7 Part 2: {}", part_2_result);
}
