struct Line {
    contents: String,
}

impl Line {
    fn new(input: &str) -> Line {
        Line {
            contents: String::from(input),
        }
    }

    fn get_first_illegal_character(&self) -> Option<char> {
        let mut stack: Vec<char> = Vec::new();

        for chunk_element in self.contents.chars() {
            match chunk_element {
                '(' | '[' | '{' | '<' => stack.push(chunk_element),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        return Some(chunk_element);
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        return Some(chunk_element);
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        return Some(chunk_element);
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        return Some(chunk_element);
                    }
                }
                _ => panic!("Unknown character: {}", chunk_element),
            }
        }

        None
    }

    // assumes no errors
    fn get_autocomplete_characters_in_order(&self) -> Vec<char> {
        let mut stack: Vec<char> = Vec::new();

        for chunk_element in self.contents.chars() {
            match chunk_element {
                '(' | '[' | '{' | '<' => stack.push(chunk_element),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => panic!("Unknown character: {}", chunk_element),
            }
        }

        stack
            .iter()
            .rev()
            .map(|chunk_element| Self::reverse_chunk_element(*chunk_element))
            .collect()
    }

    fn reverse_chunk_element(chunk_element: char) -> char {
        match chunk_element {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Unknown character: {}", chunk_element),
        }
    }
}

pub struct SyntaxChecker {
    lines: Vec<Line>,
}

impl SyntaxChecker {
    pub fn new(input: &[String]) -> SyntaxChecker {
        SyntaxChecker {
            lines: input.iter().map(|s| Line::new(s)).collect(),
        }
    }

    pub fn get_total_error_score(&self) -> u64 {
        let mut result = 0;

        for line in &self.lines {
            if let Some(error_char) = line.get_first_illegal_character() {
                match error_char {
                    ')' => result += 3,
                    ']' => result += 57,
                    '}' => result += 1197,
                    '>' => result += 25137,
                    _ => panic!("Unknown character: {}", error_char),
                }
            }
        }

        result
    }

    pub fn get_autocomplete_score(&self) -> u64 {
        let mut results: Vec<u64> = self
            .lines
            .iter()
            .filter(|line| line.get_first_illegal_character().is_none())
            .map(|line| line.get_autocomplete_characters_in_order())
            .map(|autocomplete_result| Self::score_autocomplete_result(&autocomplete_result))
            .collect();

        let median_index = results.len() / 2;

        results.sort();

        *results.get(median_index).expect("Could not get median!")
    }

    fn score_autocomplete_result(autocomplete_result: &[char]) -> u64 {
        autocomplete_result
            .iter()
            .map(|character| match character {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unknown character: {}", character),
            })
            .fold(0, |acc, score| (acc * 5) + score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_line_get_first_illegal_character_some() {
        let test_line = Line::new(TEST_DATA[2]);

        let expected = Some('}');

        let result = test_line.get_first_illegal_character();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_get_first_illegal_character_none() {
        let test_line = Line::new(TEST_DATA[0]);

        let expected = None;

        let result = test_line.get_first_illegal_character();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_get_autocomplete_characters_in_order() {
        let test_line = Line::new(TEST_DATA[0]);

        let expected = vec!['}', '}', ']', ']', ')', '}', ')', ']'];

        let result = test_line.get_autocomplete_characters_in_order();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_syntax_checker_get_total_error_score() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let test_checker = SyntaxChecker::new(&input);

        let expected = 26397;

        let result = test_checker.get_total_error_score();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_syntax_checker_get_autocomplete_score() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let test_checker = SyntaxChecker::new(&input);

        let expected = 288957;

        let result = test_checker.get_autocomplete_score();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_syntax_checker_score_autocomplete_result() {
        let input: Vec<char> = "}}>}>))))".chars().collect();

        let expected = 1480781;

        let result = SyntaxChecker::score_autocomplete_result(&input);

        assert_eq!(result, expected);
    }
}
