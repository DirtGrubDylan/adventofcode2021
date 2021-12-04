use crate::util::point_2d::Point2d;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct BingoBoard {
    numbers_to_locations: HashMap<u32, Point2d<u32>>,
    marked_columns: HashMap<u32, HashSet<u32>>,
    marked_rows: HashMap<u32, HashSet<u32>>,
    winning_number: Option<u32>,
    sum_of_uncalled_numbers: u32,
}

impl BingoBoard {
    pub fn new(input: &[String]) -> BingoBoard {
        let int_input: Vec<Vec<u32>> = input
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(' ')
                    .filter(|split| !split.is_empty())
                    .map(|split| {
                        split
                            .parse::<u32>()
                            .expect(&format!("Cannot parse {} to a u32", split))
                    })
                    .collect()
            })
            .collect();

        Self::new_from_ints(&int_input)
    }

    pub fn new_from_ints(input: &[Vec<u32>]) -> BingoBoard {
        let mut numbers_to_locations: HashMap<u32, Point2d<u32>> = HashMap::new();

        for (row_index, row) in input.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                let point = Point2d::new(column_index as u32, row_index as u32);

                numbers_to_locations.insert(*value, point);
            }
        }

        let sum = input
            .iter()
            .flat_map(|v| v.iter())
            .fold(0, |acc, x| acc + x);

        BingoBoard {
            numbers_to_locations: numbers_to_locations,
            marked_columns: HashMap::new(),
            marked_rows: HashMap::new(),
            winning_number: None,
            sum_of_uncalled_numbers: sum,
        }
    }

    pub fn mark_number(&mut self, called_number: u32) {
        let possible_location = self.numbers_to_locations.get(&called_number);

        if let Some(location) = possible_location {
            let column = location.x;
            let row = location.y;

            let marked_numbers_in_column =
                self.marked_columns.entry(column).or_insert(HashSet::new());
            let marked_numbers_in_row = self.marked_rows.entry(row).or_insert(HashSet::new());

            marked_numbers_in_column.insert(called_number);
            marked_numbers_in_row.insert(called_number);

            if (marked_numbers_in_column.len() == 5) || (marked_numbers_in_row.len() == 5) {
                self.winning_number = Some(called_number);
            }

            self.sum_of_uncalled_numbers -= called_number;
        }
    }

    pub fn is_won(&self) -> bool {
        self.winning_number.is_some()
    }

    pub fn get_score(&self) -> Option<u32> {
        self.winning_number
            .map(|winning_number| winning_number * self.sum_of_uncalled_numbers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CALLED_NUMBERS: [u32; 12] = [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
    const TEST_DATA: [&str; 6] = [
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        "2  0 12  3  7",
        "",
    ];

    #[test]
    fn test_new() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = get_test_board();

        let result = BingoBoard::new(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mark_number_does_not_exist() {
        let mut test_board = get_test_board();

        test_board.mark_number(69);

        let expected_marked_columns = HashMap::new();
        let expected_marked_rows = HashMap::new();

        assert_eq!(test_board.marked_columns, expected_marked_columns);
        assert_eq!(test_board.marked_rows, expected_marked_rows);
    }

    #[test]
    fn test_mark_number_exists() {
        let mut test_board = get_test_board();

        test_board.mark_number(11);

        let mut expected_marked_numbers = HashSet::new();
        let mut expected_marked_columns = HashMap::new();
        let mut expected_marked_rows = HashMap::new();

        expected_marked_numbers.insert(11);

        expected_marked_columns.insert(1, expected_marked_numbers.clone());
        expected_marked_rows.insert(3, expected_marked_numbers.clone());

        assert_eq!(test_board.marked_columns, expected_marked_columns);
        assert_eq!(test_board.marked_rows, expected_marked_rows);
    }

    #[test]
    fn test_mark_number_for_win() {
        let mut test_board = get_test_board();

        for number in TEST_CALLED_NUMBERS {
            test_board.mark_number(number);
        }

        let expected_score = 4512;

        assert!(test_board.is_won());
        assert_eq!(test_board.get_score(), Some(expected_score));
    }

    fn get_test_board() -> BingoBoard {
        let numbers = vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ];

        BingoBoard::new_from_ints(&numbers)
    }
}
