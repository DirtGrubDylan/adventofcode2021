#![allow(unused_imports)]

mod lanternfish;

use crate::util::file_reader::to_string_vector;

use lanternfish::LanternFish;

pub fn run_day_6() {
    let file_input = to_string_vector("inputs/day_6.txt").unwrap();

    let initial_fish =
        lanternfish::get_initial_fish(&file_input.get(0).expect(&format!("Input is empty!")));

    let part_1_result = lanternfish::get_lanternfish_population_created_in_days(&initial_fish, 80);
    let part_2_result = lanternfish::get_lanternfish_population_created_in_days(&initial_fish, 256);

    println!("Day 6 Part 1: {}", part_1_result);
    println!("Day 6 Part 2: {}", part_2_result);
}
