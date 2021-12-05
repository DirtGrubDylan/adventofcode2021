use crate::util::line_2d::Line2d;
use crate::util::point_2d::Point2d;
use std::collections::HashMap;

pub struct HydrothermalVentDiagram {
    diagram: HashMap<Point2d<i32>, u32>,
    number_of_overlapping_vents: u32,
}

impl HydrothermalVentDiagram {
    pub fn new() -> HydrothermalVentDiagram {
        HydrothermalVentDiagram {
            diagram: HashMap::new(),
            number_of_overlapping_vents: 0,
        }
    }

    pub fn add_lines<P>(&mut self, lines: &[String], predicate: P)
    where
        P: FnMut(&Line2d) -> bool,
    {
        lines
            .iter()
            .map(|line_str| Self::get_line(line_str))
            .filter(predicate)
            .for_each(|line| self.add_line(line));
    }

    pub fn get_number_of_overlapping_vents(&self) -> u32 {
        self.number_of_overlapping_vents
    }

    fn add_line(&mut self, line: Line2d) {
        for point in line.int_points_along_line() {
            let entry = self
                .diagram
                .entry(point)
                .and_modify(|x| *x += 1)
                .or_insert(1);

            if *entry == 2 {
                self.number_of_overlapping_vents += 1;
            }
        }
    }

    fn get_line(description: &str) -> Line2d {
        let (start_str, end_str) = description
            .split_once(" -> ")
            .expect(&format!("Could not split: {}", description));

        let start = Self::get_point(start_str);
        let end = Self::get_point(end_str);

        Line2d::new((start.x, start.y), (end.x, end.y))
    }

    fn get_point(description: &str) -> Point2d<i32> {
        let parse_error = |str_value| format!("Could not parse: {}", str_value);

        let (x_str, y_str) = description
            .split_once(',')
            .expect(&format!("Could not split: {}", description));

        let x = x_str.parse().expect(&parse_error(x_str));
        let y = y_str.parse().expect(&parse_error(y_str));

        Point2d::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_add_lines_all() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut test_diagram = HydrothermalVentDiagram::new();

        test_diagram.add_lines(&input, |_| true);

        let expected_map_size = 39;
        let expected_number_of_overlapping_vents = 12;

        let result_map_size = test_diagram.diagram.len();
        let result_number_of_overlapping_vents = test_diagram.number_of_overlapping_vents;

        assert_eq!(result_map_size, expected_map_size);
        assert_eq!(
            result_number_of_overlapping_vents,
            expected_number_of_overlapping_vents
        );
    }

    #[test]
    fn test_add_lines_vertical_or_horizontal_only() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let mut test_diagram = HydrothermalVentDiagram::new();

        test_diagram.add_lines(&input, |line| line.is_vertical() || line.is_horizontal());

        let expected_map_size = 21;
        let expected_number_of_overlapping_vents = 5;

        let result_map_size = test_diagram.diagram.len();
        let result_number_of_overlapping_vents = test_diagram.number_of_overlapping_vents;

        assert_eq!(result_map_size, expected_map_size);
        assert_eq!(
            result_number_of_overlapping_vents,
            expected_number_of_overlapping_vents
        );
    }

    #[test]
    fn test_get_line() {
        let input = "5,5 -> 8,2";

        let expected = Line2d::new((5, 5), (8, 2));

        let result = HydrothermalVentDiagram::get_line(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_point() {
        let input = "8,2";

        let expected = Point2d::new(8, 2);

        let result = HydrothermalVentDiagram::get_point(input);

        assert_eq!(result, expected);
    }
}
