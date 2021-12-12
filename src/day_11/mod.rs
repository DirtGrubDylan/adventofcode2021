#![allow(unused_imports)]
mod octopus_simulator;

use crate::util::file_reader::to_string_vector;
use octopus_simulator::OctopusFlashSimulator;

pub fn run_day_11() {
    let file_input = to_string_vector("inputs/day_11.txt").unwrap();

    let mut simulator = OctopusFlashSimulator::new(&file_input);

    let number_of_octopi = simulator.get_number_of_octopi();

    let mut previous_number_flashed = 0;

    let mut step = 1;

    let mut part_1_result = 0;
    let mut part_2_result = 0;

    while (previous_number_flashed != number_of_octopi) || (step < 100) {
        previous_number_flashed = simulator.next().unwrap();

        if step <= 100 {
            part_1_result += previous_number_flashed;
        }

        if previous_number_flashed == number_of_octopi {
            part_2_result = step;
        }

        step += 1;
    }

    println!("Day 11 Part 1: {}", part_1_result);
    println!("Day 11 Part 2: {}", part_2_result);
}
