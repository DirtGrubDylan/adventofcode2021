mod bingo;

use crate::util::file_reader::to_string_vector;
use bingo::BingoBoard;
use std::collections::HashSet;

pub fn run_day_4() {
    let file_input = to_string_vector("inputs/day_4.txt").unwrap();

    let scores_in_order = get_board_scores_in_winning_order(&file_input);

    println!("Day 4 Part 1: {}", scores_in_order.get(0).unwrap());
    println!("Day 4 Part 2: {}", scores_in_order.last().unwrap());
}

fn get_board_scores_in_winning_order(file_input: &[String]) -> Vec<u32> {
    let mut scores = Vec::new();

    let called_numbers = get_called_numbers(&file_input);

    let mut boards = get_boards(&file_input);

    let mut won_boards = HashSet::new();

    for number in called_numbers {
        for (board_index, board) in boards.iter_mut().enumerate() {
            if won_boards.contains(&board_index) {
                continue;
            }

            board.mark_number(number);

            if board.is_won() {
                scores.push(board.get_score().expect("Won without score?"));
                won_boards.insert(board_index);
            }
        }
    }

    scores
}

fn get_called_numbers(file_input: &[String]) -> Vec<u32> {
    file_input
        .get(0)
        .expect("File input is empty!!!")
        .split(',')
        .map(|split| {
            split
                .parse()
                .expect(&format!("Cannot parse {} into u32", split))
        })
        .collect()
}

fn get_boards(file_input: &[String]) -> Vec<BingoBoard> {
    let mut boards = Vec::new();

    let skip_head_slice = &file_input[2..];

    for chunk in skip_head_slice.chunks(6) {
        boards.push(BingoBoard::new(chunk));
    }

    boards
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 19] = [
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ];

    #[test]
    fn test_get_board_scores_in_winning_order() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = vec![4512, 2192, 1924];

        let result = get_board_scores_in_winning_order(&input);

        assert_eq!(result, expected);
    }
}
