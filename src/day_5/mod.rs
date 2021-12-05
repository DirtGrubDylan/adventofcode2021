mod hydrothermal_vent_diagram;

use crate::util::file_reader::to_string_vector;
use hydrothermal_vent_diagram::HydrothermalVentDiagram;

pub fn run_day_5() {
    let file_input = to_string_vector("inputs/day_5.txt").unwrap();

    let mut diagram = HydrothermalVentDiagram::new();

    diagram.add_lines(&file_input, |line| {
        line.is_vertical() || line.is_horizontal()
    });

    let part_1_result = diagram.get_number_of_overlapping_vents();

    diagram = HydrothermalVentDiagram::new();

    diagram.add_lines(&file_input, |_| true);

    let part_2_result = diagram.get_number_of_overlapping_vents();

    println!("Day 5 Part 1: {}", part_1_result);
    println!("Day 5 Part 2: {}", part_2_result);
}
