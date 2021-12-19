mod buoyancy_interchange_transmission_system;
mod equal_to_operation_packet;
mod greater_than_operation_packet;
mod hex_converter;
mod length_id;
mod less_than_operation_packet;
mod literal_packet;
mod maximum_operation_packet;
mod minimum_operation_packet;
mod operation_packet;
mod packet;
mod packet_header;
mod product_operation_packet;
mod sum_operation_packet;

use crate::util::file_reader::to_string_vector;
use buoyancy_interchange_transmission_system::BuoyancyInterchangeTransmissionSystem;

pub fn run_day_16() {
    let file_input = to_string_vector("inputs/day_16.txt").unwrap();

    let bits = BuoyancyInterchangeTransmissionSystem::new(
        &file_input.get(0).expect("Day 16 file was empty!"),
    );

    let part_1_result = bits.get_total_version_sum();
    let part_2_result = bits.get_value();

    println!("Day 16 Part 1: {}", part_1_result);
    println!("Day 16 Part 2: {}", part_2_result);
}
