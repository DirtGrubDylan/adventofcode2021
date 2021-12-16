mod cave_map;

use crate::util::file_reader::to_string_vector;
use cave_map::CaveMap;

pub fn run_day_15() {
    let file_input = to_string_vector("inputs/day_15.txt").unwrap();

    let mut cave_map = CaveMap::from(file_input.as_slice());

    let part_1_result = cave_map.get_lowest_total_risk_level_to_exit();

    println!("Day 15 Part 1: {}", part_1_result.unwrap());

    cave_map.tile_repeat_by_five();

    let part_2_result = cave_map.get_lowest_total_risk_level_to_exit();

    println!("Day 15 Part 2: {}", part_2_result.unwrap());
}
