use crate::util::point_2d::Point2d;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

#[derive(Debug, PartialEq)]
struct TransparentPaper {
    dots: HashSet<Point2d<i32>>,
}

impl TransparentPaper {
    fn new() -> TransparentPaper {
        TransparentPaper {
            dots: HashSet::new(),
        }
    }

    fn add_dot(&mut self, location: Point2d<i32>) {
        self.dots.insert(location);
    }

    fn fold(&mut self, fold: Fold) {
        self.dots = self
            .dots
            .drain()
            .map(|location| Self::get_fold_location(&fold, &location))
            .collect();
    }

    fn get_number_of_dots(&self) -> usize {
        self.dots.len()
    }

    fn get_fold_location(fold: &Fold, location: &Point2d<i32>) -> Point2d<i32> {
        match *fold {
            Fold::Vertical(fold_value) => {
                if fold_value < location.x {
                    Point2d::new(Self::get_folded_value(location.x, fold_value), location.y)
                } else {
                    *location
                }
            }
            Fold::Horizontal(fold_value) => {
                if fold_value < location.y {
                    Point2d::new(location.x, Self::get_folded_value(location.y, fold_value))
                } else {
                    *location
                }
            }
        }
    }

    fn get_folded_value(value: i32, fold_value: i32) -> i32 {
        (fold_value - (value % fold_value)) % fold_value
    }

    fn get_furthest_location(&self) -> Point2d<i32> {
        *self.dots.iter().max().unwrap()
    }
}

pub fn get_number_of_dots_at_each_fold(instructions: &[String]) -> Vec<usize> {
    let mut paper = TransparentPaper::new();
    let mut folds = Vec::new();

    for instruction in instructions.iter().filter(|s| !s.is_empty()) {
        if instruction.starts_with("fold") {
            folds.push(get_fold(instruction));
        } else {
            paper.add_dot(get_point(instruction));
        }
    }

    folds
        .into_iter()
        .map(|fold| {
            paper.fold(fold);

            paper.get_number_of_dots()
        })
        .collect()
}

pub fn display_transparent_paper(instructions: &[String]) {
    let mut paper = TransparentPaper::new();
    let mut folds = Vec::new();

    for instruction in instructions.iter().filter(|s| !s.is_empty()) {
        if instruction.starts_with("fold") {
            folds.push(get_fold(instruction));
        } else {
            paper.add_dot(get_point(instruction));
        }
    }

    folds.into_iter().for_each(|fold| paper.fold(fold));

    let furthest_location = paper.get_furthest_location();

    let row_template = vec![' '; (furthest_location.x + 1) as usize];
    let mut display_grid = vec![row_template.clone(); (furthest_location.y + 1) as usize];

    for location in paper.dots.iter() {
        let row = display_grid
            .get_mut(location.y as usize)
            .expect(&format!("Could not get row: {}", location.y));

        let element = row
            .get_mut(location.x as usize)
            .expect(&format!("Could not get element at: {:?}", location));

        *element = '#';
    }

    for row in display_grid {
        for element in row {
            print!("{}", element);
        }

        print!("\n");
    }
}

fn get_point(input_line: &str) -> Point2d<i32> {
    let (x_str, y_str) = input_line
        .split_once(',')
        .expect(&format!("Could not split: {}", input_line));

    let x = x_str
        .parse()
        .expect(&format!("Could not parse x: {}", x_str));
    let y = y_str
        .parse()
        .expect(&format!("Could not parse y: {}", y_str));

    Point2d::new(x, y)
}

fn get_fold(input_line: &str) -> Fold {
    let (fold_indicator, fold_value_str) = input_line[11..]
        .split_once('=')
        .expect(&format!("Could not split: {}", input_line));

    let fold_value: i32 = fold_value_str
        .parse()
        .expect(&format!("Could not parse fold value: {}", fold_value_str));

    match fold_indicator {
        "y" => Fold::Horizontal(fold_value),
        "x" => Fold::Vertical(fold_value),
        _ => panic!("Could not match {} to a fold!", fold_indicator),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TEST_DATA: [&str; 21] = [
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    #[test]
    fn test_transparent_paper_fold_horizontal() {
        let horizonal_fold = Fold::Horizontal(7);
        let initial_dots = vec![
            Point2d::new(6, 10),
            Point2d::new(0, 14),
            Point2d::new(9, 10),
            Point2d::new(0, 3),
            Point2d::new(10, 4),
            Point2d::new(4, 11),
            Point2d::new(6, 0),
            Point2d::new(6, 12),
            Point2d::new(4, 1),
            Point2d::new(0, 13),
            Point2d::new(10, 12),
            Point2d::new(3, 4),
            Point2d::new(3, 0),
            Point2d::new(8, 4),
            Point2d::new(1, 10),
            Point2d::new(2, 14),
            Point2d::new(8, 10),
            Point2d::new(9, 0),
        ]
        .into_iter()
        .collect();

        let mut test_paper = TransparentPaper { dots: initial_dots };

        test_paper.fold(horizonal_fold);

        let expected_dots = vec![
            Point2d::new(0, 0),
            Point2d::new(0, 1),
            Point2d::new(0, 3),
            Point2d::new(1, 4),
            Point2d::new(2, 0),
            Point2d::new(3, 0),
            Point2d::new(3, 4),
            Point2d::new(4, 1),
            Point2d::new(4, 3),
            Point2d::new(6, 0),
            Point2d::new(6, 2),
            Point2d::new(6, 4),
            Point2d::new(8, 4),
            Point2d::new(9, 0),
            Point2d::new(9, 4),
            Point2d::new(10, 2),
            Point2d::new(10, 4),
        ]
        .into_iter()
        .collect();

        assert_eq!(test_paper.dots, expected_dots);
    }

    #[test]
    fn test_transparent_paper_fold_vertical() {
        let vertical_fold = Fold::Vertical(5);
        let initial_dots = vec![
            Point2d::new(0, 0),
            Point2d::new(0, 1),
            Point2d::new(0, 3),
            Point2d::new(1, 4),
            Point2d::new(2, 0),
            Point2d::new(3, 0),
            Point2d::new(3, 4),
            Point2d::new(4, 1),
            Point2d::new(4, 3),
            Point2d::new(6, 0),
            Point2d::new(6, 2),
            Point2d::new(6, 4),
            Point2d::new(8, 4),
            Point2d::new(9, 0),
            Point2d::new(9, 4),
            Point2d::new(10, 2),
            Point2d::new(10, 4),
        ]
        .into_iter()
        .collect();

        let mut test_paper = TransparentPaper { dots: initial_dots };

        test_paper.fold(vertical_fold);

        let expected_dots = vec![
            Point2d::new(0, 0),
            Point2d::new(0, 1),
            Point2d::new(0, 2),
            Point2d::new(0, 3),
            Point2d::new(0, 4),
            Point2d::new(1, 0),
            Point2d::new(1, 4),
            Point2d::new(2, 0),
            Point2d::new(2, 4),
            Point2d::new(3, 0),
            Point2d::new(3, 4),
            Point2d::new(4, 0),
            Point2d::new(4, 1),
            Point2d::new(4, 2),
            Point2d::new(4, 3),
            Point2d::new(4, 4),
        ]
        .into_iter()
        .collect();

        assert_eq!(test_paper.dots, expected_dots);
    }
    #[test]
    fn test_get_number_of_dots_at_each_fold() {
        let instructions: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = vec![17, 16];

        let result = get_number_of_dots_at_each_fold(&instructions);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_point() {
        let expected = Point2d::new(6, 10);

        let result = get_point(TEST_DATA[0]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_fold() {
        let expected_horizonal_fold = Fold::Horizontal(7);
        let expected_vertical_fold = Fold::Vertical(5);

        let result_horizontal_fold = get_fold(TEST_DATA[19]);
        let result_vertical_fold = get_fold(TEST_DATA[20]);

        assert_eq!(result_horizontal_fold, expected_horizonal_fold);
        assert_eq!(result_vertical_fold, expected_vertical_fold);
    }
}
