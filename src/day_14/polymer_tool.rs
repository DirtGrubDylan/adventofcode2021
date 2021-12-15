use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct PolymerTool {
    template: Vec<char>,
    rules: HashMap<(char, char), (char, char, char)>,
}

impl PolymerTool {
    pub fn new() -> PolymerTool {
        PolymerTool {
            template: Vec::new(),
            rules: HashMap::new(),
        }
    }

    pub fn add_template(&mut self, template: &str) {
        self.template = template.chars().collect();
    }

    pub fn add_rule(&mut self, rule: &str) {
        let rule = Self::parse_rule(rule);

        self.rules.insert(rule.0, rule.1);
    }

    pub fn get_element_quantities_after_n_steps(&self, steps: usize) -> HashMap<char, usize> {
        let mut result = HashMap::new();
        let mut working_map = HashMap::new();

        for window in self.template.windows(2) {
            let tuple = Self::parse_window_to_tuple(window);

            working_map
                .entry(tuple)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        for _ in 0..steps {
            let mut temp_map = HashMap::new();

            for (tuple, count) in working_map.iter() {
                let insertion_result = self.rules.get(&tuple).unwrap();

                let tuple_1 = (insertion_result.0, insertion_result.1);
                let tuple_2 = (insertion_result.1, insertion_result.2);

                temp_map
                    .entry(tuple_1)
                    .and_modify(|e| *e += count)
                    .or_insert(*count);

                temp_map
                    .entry(tuple_2)
                    .and_modify(|e| *e += count)
                    .or_insert(*count);
            }

            working_map = temp_map;
        }

        for ((first_char, _), count) in working_map {
            result
                .entry(first_char)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }

        result
            .entry(*self.template.last().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);

        result
    }

    fn parse_window_to_tuple(window: &[char]) -> (char, char) {
        (*window.get(0).unwrap(), *window.get(1).unwrap())
    }

    fn parse_rule(rule: &str) -> ((char, char), (char, char, char)) {
        let (lhs, rhs) = rule
            .split_once(" -> ")
            .expect(&format!("Could not parse rule: {}", rule));

        let (lhs_char_1, lhs_char_2) = (
            lhs.chars()
                .nth(0)
                .expect(&format!("LHS of Rule empty: {}", rule)),
            lhs.chars()
                .nth(1)
                .expect(&format!("LHS of Rule only one character: {}", rule)),
        );

        let rhs_char = rhs
            .chars()
            .nth(0)
            .expect(&format!("RHS of Rule empty: {}", rule));

        ((lhs_char_1, lhs_char_2), (lhs_char_1, rhs_char, lhs_char_2))
    }
}

pub fn get_template_and_rules(input: &[String]) -> (String, Vec<String>) {
    let template = input.get(0).expect("Input is empty!").to_string();

    let rules = input
        .iter()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    (template, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 18] = [
        "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
        "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
        "CN -> C",
    ];

    #[test]
    fn test_parse_rule() {
        let expected = (('C', 'H'), ('C', 'B', 'H'));

        let result = PolymerTool::parse_rule(TEST_DATA[2]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_element_quantities_after_n_steps_1() {
        let test_tool = get_test_tool();

        let expected = vec![('B', 2), ('C', 2), ('H', 1), ('N', 2)]
            .into_iter()
            .collect();

        let result = test_tool.get_element_quantities_after_n_steps(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_element_quantities_after_n_steps_10() {
        let test_tool = get_test_tool();

        let expected = vec![('B', 1749), ('C', 298), ('H', 161), ('N', 865)]
            .into_iter()
            .collect();

        let result = test_tool.get_element_quantities_after_n_steps(10);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_element_quantities_after_n_steps_40() {
        let test_tool = get_test_tool();

        let elements = test_tool.get_element_quantities_after_n_steps(40);

        let expected_max = 2192039569602;
        let expected_min = 3849876073;

        let result_max = *elements.values().max().unwrap();
        let result_min = *elements.values().min().unwrap();

        assert_eq!(result_max, expected_max);
        assert_eq!(result_min, expected_min);
    }

    #[test]
    fn test_get_template_and_rules() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected_template = TEST_DATA[0].to_string();
        let expected_rules = TEST_DATA[2..].iter().map(|s| s.to_string()).collect();

        let expected = (expected_template, expected_rules);

        let result = get_template_and_rules(&input);

        assert_eq!(result, expected);
    }

    fn get_test_tool() -> PolymerTool {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let (template, rules) = get_template_and_rules(&input);

        let mut test_tool = PolymerTool::new();

        test_tool.add_template(&template);

        rules.iter().for_each(|rule| test_tool.add_rule(rule));

        test_tool
    }
}
