use crate::util::location::Location;
use crate::util::point_2d::Point2d;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const RELATIVE_NEIGHBOR_POINTS: [Point2d<i32>; 4] = [
    Point2d { x: 0, y: 1 },
    Point2d { x: 1, y: 0 },
    Point2d { x: 0, y: -1 },
    Point2d { x: -1, y: 0 },
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct RiskLevel {
    level: u32,
    position: Point2d<i32>,
}

impl RiskLevel {
    fn new(position: Point2d<i32>, level: u32) -> RiskLevel {
        RiskLevel { level, position }
    }

    fn increase_risk(&self, increase: u32) -> RiskLevel {
        RiskLevel {
            level: self.level + increase,
            position: self.position,
        }
    }
}

impl PartialOrd for RiskLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RiskLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .level
            .cmp(&self.level)
            .then_with(|| self.position.cmp(&other.position))
    }
}

#[derive(Debug, PartialEq)]
pub struct CaveMap {
    risk_level_map: HashMap<Point2d<i32>, u32>,
    number_of_rows: i32,
    number_of_columns: i32,
}

impl CaveMap {
    pub fn new() -> CaveMap {
        CaveMap {
            risk_level_map: HashMap::new(),
            number_of_rows: 0,
            number_of_columns: 0,
        }
    }

    pub fn add_risk_levels(&mut self, risk_levels: &[Vec<u32>]) {
        self.number_of_columns = risk_levels.len() as i32;
        self.number_of_rows = risk_levels.len() as i32;

        for (y, row) in risk_levels.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                let point = Point2d::new(x as i32, y as i32);

                self.risk_level_map.insert(point, *element);
            }
        }
    }

    pub fn tile_repeat_by_five(&mut self) {
        let mut temp_map = HashMap::new();

        for (point, level) in self.risk_level_map.iter() {
            for y_increase in 0..5 {
                for x_increase in 0..5 {
                    let new_x = point.x + x_increase * self.number_of_columns;
                    let new_y = point.y + y_increase * self.number_of_rows;

                    let new_point = Point2d::new(new_x, new_y);

                    let new_level = ((y_increase + x_increase) as u32 + *level - 1) % 9 + 1;

                    temp_map.insert(new_point, new_level);
                }
            }
        }

        self.risk_level_map = temp_map;
        self.number_of_columns *= 5;
        self.number_of_rows *= 5;
    }

    pub fn get_lowest_total_risk_level_to_exit(&self) -> Option<u32> {
        let (max_x, max_y) = (self.number_of_columns - 1, self.number_of_rows - 1);

        let mut seen_points: HashSet<Point2d<i32>> = HashSet::new();
        let mut risk_levels = BinaryHeap::new();

        let starting_risk_level = RiskLevel::new(Point2d::new(0, 0), 0);

        risk_levels.push(starting_risk_level);

        while !risk_levels.is_empty() {
            let current_risk_level = risk_levels.pop().unwrap();

            let current_position = current_risk_level.position;

            if seen_points.contains(&current_position) {
                continue;
            }

            if (current_position.x == max_x) && (current_position.y == max_y) {
                return Some(current_risk_level.level);
            }

            seen_points.insert(current_risk_level.position);

            RELATIVE_NEIGHBOR_POINTS
                .iter()
                .map(|relative_point| current_position.add(relative_point))
                .map(|point| self.get_risk_level_at_point(&point))
                .filter_map(|risk_level| risk_level)
                .filter(|risk_level| !seen_points.contains(&risk_level.position))
                .map(|risk_level| risk_level.increase_risk(current_risk_level.level))
                .for_each(|risk_level| risk_levels.push(risk_level));
        }

        None
    }

    fn get_risk_level_at_point(&self, point: &Point2d<i32>) -> Option<RiskLevel> {
        self.risk_level_map
            .get(point)
            .map(|level| RiskLevel::new(*point, *level))
    }
}

impl From<&[String]> for CaveMap {
    fn from(input: &[String]) -> CaveMap {
        let mut cave_map = CaveMap::new();

        let risk_levels: Vec<Vec<u32>> = input
            .iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        cave_map.add_risk_levels(&risk_levels);

        cave_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_tools::str_slice_to_string_vector;

    const TEST_DATA_SMALL: [&str; 2] = ["16", "20"];
    const TEST_DATA: [&str; 10] = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn test_from() {
        let input = str_slice_to_string_vector(&TEST_DATA_SMALL);

        let expected = CaveMap {
            risk_level_map: vec![
                (Point2d::new(0, 0), 1),
                (Point2d::new(1, 0), 6),
                (Point2d::new(0, 1), 2),
                (Point2d::new(1, 1), 0),
            ]
            .into_iter()
            .collect(),
            number_of_rows: 2,
            number_of_columns: 2,
        };

        let result = CaveMap::from(input.as_slice());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_lowest_total_risk_level_to_exit() {
        let input = str_slice_to_string_vector(&TEST_DATA);

        let cave_map = CaveMap::from(input.as_slice());

        let expected = Some(40);

        let result = cave_map.get_lowest_total_risk_level_to_exit();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_lowest_total_risk_level_to_exit_tile_repeat_five() {
        let input = str_slice_to_string_vector(&TEST_DATA);

        let mut cave_map = CaveMap::from(input.as_slice());

        cave_map.tile_repeat_by_five();

        let expected = Some(315);

        let result = cave_map.get_lowest_total_risk_level_to_exit();

        assert_eq!(result, expected);
    }
}
