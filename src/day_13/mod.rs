mod transparent_paper;

use crate::util::file_reader::to_string_vector;
use transparent_paper::{display_transparent_paper, get_number_of_dots_at_each_fold};

pub fn run_day_13() {
    let file_input = to_string_vector("inputs/day_13.txt").unwrap();

    let number_of_dots_at_each_fold = get_number_of_dots_at_each_fold(&file_input);

    let part_1_result = number_of_dots_at_each_fold
        .first()
        .expect("Array is empty!");

    println!("Day 13 Part 1: {}", part_1_result);
    println!("Day 13 Part 2:");
    display_transparent_paper(&file_input);
}
