#![allow(unused_imports)]
mod smoke_basin;

use crate::util::file_reader::to_string_vector;

use smoke_basin::CaveFloor;

pub fn run_day_9() {
    let file_input = to_string_vector("inputs/day_9.txt").unwrap();

    let mut cave_floor = CaveFloor::new();

    cave_floor.set_height_map(&file_input);

    let part_1_result: u32 = cave_floor.get_risk_levels().iter().sum();
    let part_2_result: usize = cave_floor
        .get_three_largest_basins()
        .iter()
        .fold(1, |acc, basin| acc * basin.len());

    println!("Day 9 Part 1: {}", part_1_result);
    println!("Day 9 Part 2: {}", part_2_result);
}
