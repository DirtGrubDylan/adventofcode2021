#![allow(unused_imports)]
mod syntax_checker;

use std::os::windows::fs::symlink_file;

use crate::util::file_reader::to_string_vector;
use syntax_checker::SyntaxChecker;

pub fn run_day_10() {
    let file_input = to_string_vector("inputs/day_10.txt").unwrap();

    let checker = SyntaxChecker::new(&file_input);

    let part_1_result = checker.get_total_error_score();
    let part_2_result = checker.get_autocomplete_score();

    println!("Day 10 Part 1: {}", part_1_result);
    println!("Day 10 Part 2: {}", part_2_result);
}
