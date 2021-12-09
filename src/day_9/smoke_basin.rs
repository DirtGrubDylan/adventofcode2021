use std::collections::{HashSet, VecDeque};

pub struct CaveFloor {
    height_map: Vec<Vec<u32>>,
}

impl CaveFloor {
    pub fn new() -> CaveFloor {
        CaveFloor {
            height_map: Vec::new(),
        }
    }

    pub fn set_height_map(&mut self, input: &[String]) {
        self.height_map = input.iter().map(|line| Self::string_to_vec(line)).collect();
    }

    pub fn get_risk_levels(&self) -> Vec<u32> {
        self.get_low_points()
            .iter()
            .map(|&(column, row)| self.get_height_at(column, row) + 1)
            .collect()
    }

    pub fn get_three_largest_basins(&self) -> Vec<HashSet<(usize, usize)>> {
        let mut basins = self.get_basins();

        basins.sort_by(|basin_1, basin_2| basin_1.len().cmp(&basin_2.len()));

        basins.reverse();

        basins.into_iter().take(3).collect()
    }

    fn get_basins(&self) -> Vec<HashSet<(usize, usize)>> {
        self.get_low_points()
            .iter()
            .map(|&(column, row)| self.get_basin_for_low_point(column, row))
            .collect()
    }

    fn get_basin_for_low_point(
        &self,
        low_point_column: usize,
        low_point_row: usize,
    ) -> HashSet<(usize, usize)> {
        let mut seen_locations = HashSet::new();
        let mut points_to_check = VecDeque::from([(low_point_column, low_point_row)]);

        while !points_to_check.is_empty() {
            let (column, row) = points_to_check.pop_front().expect("Queue was empty!");

            if let Some(height) = self.get_height_above(column, row) {
                if height != 9 && !seen_locations.contains(&(column, row - 1)) {
                    points_to_check.push_back((column, row - 1));
                }
            }

            if let Some(height) = self.get_height_below(column, row) {
                if height != 9 && !seen_locations.contains(&(column, row + 1)) {
                    points_to_check.push_back((column, row + 1));
                }
            }

            if let Some(height) = self.get_height_left_of(column, row) {
                if height != 9 && !seen_locations.contains(&(column - 1, row)) {
                    points_to_check.push_back((column - 1, row));
                }
            }

            if let Some(height) = self.get_height_right_of(column, row) {
                if height != 9 && !seen_locations.contains(&(column + 1, row)) {
                    points_to_check.push_back((column + 1, row));
                }
            }

            seen_locations.insert((column, row));
        }

        seen_locations
    }

    fn get_low_points(&self) -> HashSet<(usize, usize)> {
        let row_size = self.height_map.len();
        let column_size = self.height_map.get(0).expect("Height map is empty!").len();

        let mut result = HashSet::new();

        for row_index in 0..row_size {
            for column_index in 0..column_size {
                if self.is_low_point(column_index, row_index) {
                    result.insert((column_index, row_index));
                }
            }
        }

        result
    }

    fn is_low_point(&self, column: usize, row: usize) -> bool {
        let height_at_point = self.get_height_at(column, row);

        let mut result = true;

        if height_at_point == 9 {
            return false;
        }

        if let Some(height) = self.get_height_above(column, row) {
            result &= height_at_point < height;
        }

        if let Some(height) = self.get_height_below(column, row) {
            result &= height_at_point < height;
        }

        if let Some(height) = self.get_height_left_of(column, row) {
            result &= height_at_point < height;
        }

        if let Some(height) = self.get_height_right_of(column, row) {
            result &= height_at_point < height;
        }

        result
    }

    fn get_height_above(&self, column: usize, row: usize) -> Option<u32> {
        if row != 0 {
            Some(self.get_height_at(column, row - 1))
        } else {
            None
        }
    }

    fn get_height_below(&self, column: usize, row: usize) -> Option<u32> {
        let row_size = self.height_map.len();

        if row != (row_size - 1) {
            Some(self.get_height_at(column, row + 1))
        } else {
            None
        }
    }

    fn get_height_left_of(&self, column: usize, row: usize) -> Option<u32> {
        if column != 0 {
            Some(self.get_height_at(column - 1, row))
        } else {
            None
        }
    }

    fn get_height_right_of(&self, column: usize, row: usize) -> Option<u32> {
        let column_size = self.height_map.get(0).expect("Height map is empty!").len();

        if column != (column_size - 1) {
            Some(self.get_height_at(column + 1, row))
        } else {
            None
        }
    }

    fn get_height_at(&self, column: usize, row: usize) -> u32 {
        *self
            .height_map
            .get(row)
            .expect(&format!("Could not get row: {}", row))
            .get(column)
            .expect(&format!("Could not get column: {}", column))
    }

    fn string_to_vec(input: &str) -> Vec<u32> {
        input
            .chars()
            .map(|c| c.to_digit(10).expect(&format!("Could not parse: {}", c)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 5] = [
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    #[test]
    fn test_cave_floor_get_risk_levels() {
        let cave_floor = get_test_cave_floor();

        let expected = 15;

        let result: u32 = cave_floor.get_risk_levels().iter().sum();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cave_floor_is_low_point() {
        let cave_floor = get_test_cave_floor();

        let expected_corner_1 = false;
        let expected_corner_2 = true;
        let expected_edge_1 = false;
        let expected_edge_2 = true;
        let expected_center_1 = false;
        let expected_center_2 = true;

        let result_corner_1 = cave_floor.is_low_point(0, 0);
        let result_corner_2 = cave_floor.is_low_point(9, 0);
        let result_edge_1 = cave_floor.is_low_point(0, 2);
        let result_edge_2 = cave_floor.is_low_point(6, 4);
        let result_center_1 = cave_floor.is_low_point(1, 1);
        let result_center_2 = cave_floor.is_low_point(2, 2);

        assert_eq!(result_corner_1, expected_corner_1);
        assert_eq!(result_corner_2, expected_corner_2);
        assert_eq!(result_edge_1, expected_edge_1);
        assert_eq!(result_edge_2, expected_edge_2);
        assert_eq!(result_center_1, expected_center_1);
        assert_eq!(result_center_2, expected_center_2);
    }

    #[test]
    fn test_cave_floor_get_basin_for_low_point() {
        let cave_floor = get_test_cave_floor();

        let expected = [
            (9, 0),
            (8, 0),
            (7, 0),
            (6, 0),
            (5, 0),
            (6, 1),
            (8, 1),
            (9, 1),
            (9, 2),
        ]
        .into_iter()
        .collect();

        let result = cave_floor.get_basin_for_low_point(9, 0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cave_floor_get_three_largest_basins() {
        let cave_floor = get_test_cave_floor();

        let expected = 1134;

        let result = cave_floor
            .get_three_largest_basins()
            .iter()
            .fold(1, |acc, basin| acc * basin.len());

        assert_eq!(result, expected);
    }

    fn get_test_cave_floor() -> CaveFloor {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut cave_floor = CaveFloor::new();

        cave_floor.set_height_map(&input);

        cave_floor
    }
}
