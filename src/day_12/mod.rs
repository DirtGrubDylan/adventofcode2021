#![allow(unused_imports)]
mod cave_system;

use crate::util::file_reader::to_string_vector;

use cave_system::CaveSystem;

pub fn run_day_12() {
    let file_input = to_string_vector("inputs/day_12.txt").unwrap();

    let cave_system = CaveSystem::new_from(&file_input);

    let part_1_result = cave_system.number_of_paths_to_end_visiting_small_caves_once();
    let part_2_result = cave_system.number_of_paths_to_end_visiting_small_caves_once_maybe_twice();

    println!("Day 12 Part 1: {}", part_1_result);
    println!("Day 12 Part 2: {}", part_2_result);
}
