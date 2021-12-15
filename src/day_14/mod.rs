mod polymer_tool;

use crate::util::file_reader::to_string_vector;
use polymer_tool::{get_template_and_rules, PolymerTool};

pub fn run_day_14() {
    let file_input = to_string_vector("inputs/day_14.txt").unwrap();

    let polymer_tool = get_tool(&file_input);

    let part_1_result = get_result_after_n_steps(&polymer_tool, 10);
    let part_2_result = get_result_after_n_steps(&polymer_tool, 40);

    println!("Day 14 Part 1: {}", part_1_result);
    println!("Day 14 Part 2: {}", part_2_result);
}

fn get_tool(input: &[String]) -> PolymerTool {
    let (template, rules) = get_template_and_rules(&input);

    let mut polymer_tool = PolymerTool::new();

    polymer_tool.add_template(&template);

    rules.iter().for_each(|rule| polymer_tool.add_rule(rule));

    polymer_tool
}

fn get_result_after_n_steps(tool: &PolymerTool, steps: usize) -> usize {
    let element_counts = tool.get_element_quantities_after_n_steps(steps);

    let max_value = element_counts.values().max().unwrap();
    let min_value = element_counts.values().min().unwrap();

    max_value - min_value
}
