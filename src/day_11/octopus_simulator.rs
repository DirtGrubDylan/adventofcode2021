use crate::util::{location::Location, point_2d::Point2d};
use std::collections::{HashMap, HashSet, VecDeque};

const RELATIVE_NEIGHBOR_LOCATIONS: [Point2d<i32>; 8] = [
    Point2d { x: 0, y: 1 },
    Point2d { x: 1, y: 1 },
    Point2d { x: 1, y: 0 },
    Point2d { x: 1, y: -1 },
    Point2d { x: 0, y: -1 },
    Point2d { x: -1, y: -1 },
    Point2d { x: -1, y: 0 },
    Point2d { x: -1, y: 1 },
];

#[derive(Debug, PartialEq)]
struct Octopus {
    energy_level: u32,
}

impl Octopus {
    fn new_from(energy_level_char: char) -> Octopus {
        Octopus {
            energy_level: energy_level_char.to_digit(10).expect(&format!(
                "Could not parse energy level: {}",
                energy_level_char
            )),
        }
    }

    fn increase_energy_level(&mut self, increase: u32) -> u32 {
        self.energy_level += increase;

        self.energy_level
    }

    fn flash(&mut self) -> u32 {
        self.energy_level = 0;

        1
    }
}

#[derive(Debug, PartialEq)]
pub struct OctopusFlashSimulator {
    octopi: HashMap<Point2d<i32>, Octopus>,
    number_flashed_so_far: u32,
}

impl OctopusFlashSimulator {
    pub fn new(octopi_energy_levels: &[String]) -> OctopusFlashSimulator {
        let mut map = HashMap::new();

        for (row_index, row) in octopi_energy_levels.iter().enumerate() {
            for (column_index, column) in row.chars().enumerate() {
                let point = Point2d::new(column_index as i32, row_index as i32);

                let octopus = Octopus::new_from(column);

                map.insert(point, octopus);
            }
        }

        OctopusFlashSimulator {
            octopi: map,
            number_flashed_so_far: 0,
        }
    }

    pub fn get_number_of_octopi(&self) -> u32 {
        self.octopi.len() as u32
    }

    fn execute_one_step(&mut self) -> u32 {
        let mut already_flashed_octopi_locations: HashSet<Point2d<i32>> = HashSet::new();
        let mut flashing_octopi_locations: VecDeque<Point2d<i32>> = VecDeque::new();

        for (location, octopus) in self.octopi.iter_mut() {
            let octopus_energy_level = octopus.increase_energy_level(1);

            if octopus_energy_level > 9 {
                flashing_octopi_locations.push_back(*location);
            }
        }

        while !flashing_octopi_locations.is_empty() {
            let flashing_octopi_location = flashing_octopi_locations.pop_front().unwrap();

            if already_flashed_octopi_locations.contains(&flashing_octopi_location) {
                continue;
            }

            let flash_energy = self
                .octopi
                .get_mut(&flashing_octopi_location)
                .unwrap()
                .flash();

            already_flashed_octopi_locations.insert(flashing_octopi_location);

            for neighbor_location in Self::get_neighbor_locations(&flashing_octopi_location) {
                if already_flashed_octopi_locations.contains(&neighbor_location) {
                    continue;
                }

                if let Some(neighbor_octopus) = self.octopi.get_mut(&neighbor_location) {
                    let neighbor_octopus_energy_level =
                        neighbor_octopus.increase_energy_level(flash_energy);

                    if neighbor_octopus_energy_level > 9 {
                        flashing_octopi_locations.push_back(neighbor_location);
                    }
                }
            }
        }

        already_flashed_octopi_locations.len() as u32
    }

    fn get_neighbor_locations(center_location: &Point2d<i32>) -> Vec<Point2d<i32>> {
        RELATIVE_NEIGHBOR_LOCATIONS
            .iter()
            .map(|relative_location| relative_location.add(center_location))
            .collect()
    }
}

impl Iterator for OctopusFlashSimulator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.number_flashed_so_far = self.execute_one_step();

        Some(self.number_flashed_so_far)
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    const TEST_SMALL_DATA: [&str; 5] = ["11111", "19991", "19191", "19991", "11111"];

    const TEST_LARGE_DATA: [&str; 10] = [
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    #[test]
    fn test_octopus_flash_simulator_new() {
        let input: Vec<String> = TEST_SMALL_DATA.iter().map(|s| s.to_string()).collect();

        let expected_map: HashMap<Point2d<i32>, Octopus> = vec![
            (Point2d::new(0, 0), Octopus { energy_level: 1 }),
            (Point2d::new(1, 0), Octopus { energy_level: 1 }),
            (Point2d::new(2, 0), Octopus { energy_level: 1 }),
            (Point2d::new(3, 0), Octopus { energy_level: 1 }),
            (Point2d::new(4, 0), Octopus { energy_level: 1 }),
            (Point2d::new(0, 1), Octopus { energy_level: 1 }),
            (Point2d::new(1, 1), Octopus { energy_level: 9 }),
            (Point2d::new(2, 1), Octopus { energy_level: 9 }),
            (Point2d::new(3, 1), Octopus { energy_level: 9 }),
            (Point2d::new(4, 1), Octopus { energy_level: 1 }),
            (Point2d::new(0, 2), Octopus { energy_level: 1 }),
            (Point2d::new(1, 2), Octopus { energy_level: 9 }),
            (Point2d::new(2, 2), Octopus { energy_level: 1 }),
            (Point2d::new(3, 2), Octopus { energy_level: 9 }),
            (Point2d::new(4, 2), Octopus { energy_level: 1 }),
            (Point2d::new(0, 3), Octopus { energy_level: 1 }),
            (Point2d::new(1, 3), Octopus { energy_level: 9 }),
            (Point2d::new(2, 3), Octopus { energy_level: 9 }),
            (Point2d::new(3, 3), Octopus { energy_level: 9 }),
            (Point2d::new(4, 3), Octopus { energy_level: 1 }),
            (Point2d::new(0, 4), Octopus { energy_level: 1 }),
            (Point2d::new(1, 4), Octopus { energy_level: 1 }),
            (Point2d::new(2, 4), Octopus { energy_level: 1 }),
            (Point2d::new(3, 4), Octopus { energy_level: 1 }),
            (Point2d::new(4, 4), Octopus { energy_level: 1 }),
        ]
        .into_iter()
        .collect();

        let expected = OctopusFlashSimulator {
            octopi: expected_map,
            number_flashed_so_far: 0,
        };

        let result = OctopusFlashSimulator::new(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_octopus_flash_simulator_execute_one_step() {
        let input: Vec<String> = TEST_SMALL_DATA.iter().map(|s| s.to_string()).collect();

        let mut simulator = OctopusFlashSimulator::new(&input);

        let expected_number_of_flashes = 9;
        let expected_map: HashMap<Point2d<i32>, Octopus> = vec![
            (Point2d::new(0, 0), Octopus { energy_level: 3 }),
            (Point2d::new(1, 0), Octopus { energy_level: 4 }),
            (Point2d::new(2, 0), Octopus { energy_level: 5 }),
            (Point2d::new(3, 0), Octopus { energy_level: 4 }),
            (Point2d::new(4, 0), Octopus { energy_level: 3 }),
            (Point2d::new(0, 1), Octopus { energy_level: 4 }),
            (Point2d::new(1, 1), Octopus { energy_level: 0 }),
            (Point2d::new(2, 1), Octopus { energy_level: 0 }),
            (Point2d::new(3, 1), Octopus { energy_level: 0 }),
            (Point2d::new(4, 1), Octopus { energy_level: 4 }),
            (Point2d::new(0, 2), Octopus { energy_level: 5 }),
            (Point2d::new(1, 2), Octopus { energy_level: 0 }),
            (Point2d::new(2, 2), Octopus { energy_level: 0 }),
            (Point2d::new(3, 2), Octopus { energy_level: 0 }),
            (Point2d::new(4, 2), Octopus { energy_level: 5 }),
            (Point2d::new(0, 3), Octopus { energy_level: 4 }),
            (Point2d::new(1, 3), Octopus { energy_level: 0 }),
            (Point2d::new(2, 3), Octopus { energy_level: 0 }),
            (Point2d::new(3, 3), Octopus { energy_level: 0 }),
            (Point2d::new(4, 3), Octopus { energy_level: 4 }),
            (Point2d::new(0, 4), Octopus { energy_level: 3 }),
            (Point2d::new(1, 4), Octopus { energy_level: 4 }),
            (Point2d::new(2, 4), Octopus { energy_level: 5 }),
            (Point2d::new(3, 4), Octopus { energy_level: 4 }),
            (Point2d::new(4, 4), Octopus { energy_level: 3 }),
        ]
        .into_iter()
        .collect();

        let result_number_of_flashes = simulator.execute_one_step();
        let result_map = simulator.octopi;

        assert_eq!(result_number_of_flashes, expected_number_of_flashes);
        assert_eq!(result_map, expected_map);
    }

    #[test]
    fn test_octopus_flash_simulator_iter_10() {
        let input: Vec<String> = TEST_LARGE_DATA.iter().map(|s| s.to_string()).collect();

        let simulator = OctopusFlashSimulator::new(&input);

        let expected = 204;

        let result: u32 = simulator.take(10).sum();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_octopus_flash_simulator_iter_100() {
        let input: Vec<String> = TEST_LARGE_DATA.iter().map(|s| s.to_string()).collect();

        let simulator = OctopusFlashSimulator::new(&input);

        let expected = 1656;

        let result: u32 = simulator.take(100).sum();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_octopus_flash_simulator_iter_when_all_flash() {
        let input: Vec<String> = TEST_LARGE_DATA.iter().map(|s| s.to_string()).collect();

        let simulator = OctopusFlashSimulator::new(&input);

        let number_of_octopi = simulator.get_number_of_octopi();

        let expected = 195;

        let result = simulator
            .enumerate()
            .find(|(_, number_flashed)| *number_flashed == number_of_octopi)
            .unwrap()
            .0
            + 1;

        assert_eq!(result, expected);
    }
}
