use std::cmp::Ordering;

use super::point_2d::Point2d;

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Copy, Clone, Ord)]
pub struct Line2d {
    pub start: Point2d<i32>,
    pub end: Point2d<i32>,
}

impl Line2d {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Line2d {
        Line2d {
            start: Point2d::new(start.0, start.1),
            end: Point2d::new(end.0, end.1),
        }
    }

    pub fn int_points_along_line(&self) -> Vec<Point2d<i32>> {
        let mut result = Vec::new();

        let mut current_point = self.start;

        while current_point != self.end {
            result.push(current_point);

            let new_x = match current_point.x.cmp(&self.end.x) {
                Ordering::Less => current_point.x + 1,
                Ordering::Greater => current_point.x - 1,
                Ordering::Equal => current_point.x,
            };

            let new_y = match current_point.y.cmp(&self.end.y) {
                Ordering::Less => current_point.y + 1,
                Ordering::Greater => current_point.y - 1,
                Ordering::Equal => current_point.y,
            };

            current_point = Point2d::new(new_x, new_y);
        }

        result.push(self.end);

        result
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_points_along_line() {
        let test_line = Line2d::new((9, 4), (3, 4));

        let expected = vec![
            Point2d::new(9, 4),
            Point2d::new(8, 4),
            Point2d::new(7, 4),
            Point2d::new(6, 4),
            Point2d::new(5, 4),
            Point2d::new(4, 4),
            Point2d::new(3, 4),
        ];

        let result = test_line.int_points_along_line();

        assert_eq!(result, expected);
    }
}
