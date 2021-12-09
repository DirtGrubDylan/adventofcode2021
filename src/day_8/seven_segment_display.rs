use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Wire {
    fn from(input: char) -> Wire {
        match input {
            'a' => Wire::A,
            'b' => Wire::B,
            'c' => Wire::C,
            'd' => Wire::D,
            'e' => Wire::E,
            'f' => Wire::F,
            'g' => Wire::G,
            _ => panic!("Cannot convert '{}' to a wire!", input),
        }
    }
}

pub struct Display {
    wires_to_segments: HashMap<Wire, Segment>,
    number_segments: HashMap<u8, HashSet<Segment>>,
}

impl Display {
    pub fn new() -> Display {
        let mut number_segments = HashMap::new();

        number_segments.insert(
            0,
            [
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::E,
                Segment::F,
                Segment::G,
            ]
            .into_iter()
            .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            1,
            [Segment::C, Segment::F]
                .into_iter()
                .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            2,
            [Segment::A, Segment::C, Segment::D, Segment::E, Segment::G]
                .into_iter()
                .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            3,
            [Segment::A, Segment::C, Segment::D, Segment::F, Segment::G]
                .into_iter()
                .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            4,
            [Segment::B, Segment::C, Segment::D, Segment::F]
                .into_iter()
                .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            5,
            [Segment::A, Segment::B, Segment::D, Segment::F, Segment::G]
                .into_iter()
                .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            6,
            [
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ]
            .into_iter()
            .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            7,
            [Segment::A, Segment::C, Segment::F]
                .into_iter()
                .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            8,
            [
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ]
            .into_iter()
            .collect::<HashSet<Segment>>(),
        );

        number_segments.insert(
            9,
            [
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::F,
                Segment::G,
            ]
            .into_iter()
            .collect::<HashSet<Segment>>(),
        );

        Display {
            wires_to_segments: HashMap::new(),
            number_segments: number_segments,
        }
    }

    pub fn map_wires_to_segments(&mut self, unique_signal_patterns: &[String]) {
        let wires_for_1 = Self::get_1_wires(unique_signal_patterns);
        let wires_for_4 = Self::get_4_wires(unique_signal_patterns);
        let wires_for_7 = Self::get_7_wires(unique_signal_patterns);
        let wires_for_8 = Self::get_8_wires(unique_signal_patterns);

        let segment_a_wire: Wire = *wires_for_7
            .iter()
            .filter(|wire| !wires_for_1.contains(wire))
            .nth(0)
            .unwrap();

        self.wires_to_segments.insert(segment_a_wire, Segment::A);

        let segment_c_and_f_wires = wires_for_1.clone();

        let segment_b_and_d_wires: HashSet<Wire> = wires_for_4
            .iter()
            .copied()
            .filter(|wire| !wires_for_1.contains(wire))
            .collect();

        let segment_e_and_g_wires: HashSet<Wire> = wires_for_8
            .iter()
            .copied()
            .filter(|wire| !wires_for_4.contains(wire))
            .filter(|wire| !wires_for_7.contains(wire))
            .collect();

        // only handle the 6 segment numbers (0, 6, 9)
        for pattern_str in unique_signal_patterns {
            if pattern_str.len() != 6 {
                continue;
            }

            let pattern = get_unique_signal_pattern_wires(pattern_str);

            let remaining_wire = *wires_for_8
                .iter()
                .filter(|wire| !pattern.contains(wire))
                .nth(0)
                .unwrap();

            if segment_c_and_f_wires.contains(&remaining_wire) {
                let segment_f_wire: Wire = *segment_c_and_f_wires
                    .iter()
                    .filter(|&&wire| wire != remaining_wire)
                    .nth(0)
                    .unwrap();

                self.wires_to_segments.insert(remaining_wire, Segment::C);
                self.wires_to_segments.insert(segment_f_wire, Segment::F);
            } else if segment_b_and_d_wires.contains(&remaining_wire) {
                let segment_b_wire: Wire = *segment_b_and_d_wires
                    .iter()
                    .filter(|&&wire| wire != remaining_wire)
                    .nth(0)
                    .unwrap();

                self.wires_to_segments.insert(remaining_wire, Segment::D);
                self.wires_to_segments.insert(segment_b_wire, Segment::B);
            } else if segment_e_and_g_wires.contains(&remaining_wire) {
                let segment_g_wire: Wire = *segment_e_and_g_wires
                    .iter()
                    .filter(|&&wire| wire != remaining_wire)
                    .nth(0)
                    .unwrap();

                self.wires_to_segments.insert(remaining_wire, Segment::E);
                self.wires_to_segments.insert(segment_g_wire, Segment::G);
            }
        }
    }

    pub fn get_display_number(&self, live_wire_pattern: &str) -> u8 {
        let segments: HashSet<Segment> = live_wire_pattern
            .chars()
            .map(|c| Wire::from(c))
            .map(|wire| {
                self.wires_to_segments
                    .get(&wire)
                    .expect(&format!("Cannot get segment for wire: {:?}", wire))
                    .clone()
            })
            .collect();

        for (number, number_segments) in &self.number_segments {
            if segments == *number_segments {
                return *number;
            }
        }

        panic!("Could not find a number for segments: {:?}", segments);
    }

    fn get_1_wires(unique_signal_patterns: &[String]) -> HashSet<Wire> {
        get_unique_signal_pattern_wires(
            unique_signal_patterns
                .iter()
                .find(|pattern| pattern.len() == 2)
                .expect(&format!("Patterns missing 1!")),
        )
    }

    fn get_4_wires(unique_signal_patterns: &[String]) -> HashSet<Wire> {
        get_unique_signal_pattern_wires(
            unique_signal_patterns
                .iter()
                .find(|pattern| pattern.len() == 4)
                .expect(&format!("Patterns missing 4!")),
        )
    }

    fn get_7_wires(unique_signal_patterns: &[String]) -> HashSet<Wire> {
        get_unique_signal_pattern_wires(
            unique_signal_patterns
                .iter()
                .find(|pattern| pattern.len() == 3)
                .expect(&format!("Patterns missing 7!")),
        )
    }

    fn get_8_wires(unique_signal_patterns: &[String]) -> HashSet<Wire> {
        get_unique_signal_pattern_wires(
            unique_signal_patterns
                .iter()
                .find(|pattern| pattern.len() == 7)
                .expect(&format!("Patterns missing 8!")),
        )
    }
}

pub fn get_displays_for(inputs: &[String]) -> Vec<u32> {
    inputs.iter().map(|input| get_display_for(input)).collect()
}

pub fn get_number_of_1_4_7_or_8_displays(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| get_patterns_and_displays(s))
        .flat_map(|(_, displays)| displays.into_iter())
        .fold(0, |acc, display| {
            acc + match display.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        })
}

fn get_display_for(input: &str) -> u32 {
    let (patterns, display_patterns) = get_patterns_and_displays(input);

    let mut display = Display::new();

    display.map_wires_to_segments(&patterns);

    let mut result = String::new();

    for display_pattern in display_patterns {
        result += &display.get_display_number(&display_pattern).to_string();
    }

    result
        .parse()
        .expect(&format!("Could not parse u32: {}", result))
}

fn get_unique_signal_pattern_wires(unique_signal_pattern: &str) -> HashSet<Wire> {
    unique_signal_pattern
        .chars()
        .map(|c| Wire::from(c))
        .collect()
}

fn get_patterns_and_displays(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, displays) = input
        .split_once(" | ")
        .expect(&format!("Could not split ' | ' on: {}", input));

    let patterns_vec = patterns.split(' ').map(|s| s.to_string()).collect();
    let displays_vec = displays.split(' ').map(|s| s.to_string()).collect();

    (patterns_vec, displays_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    const TEST_UNIQUE_SIGNAL_PATTERNS: [&str; 10] = [
        "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
    ];
    const TEST_DISPLAY_PATTERNS: [&str; 4] = ["cdfeb", "fcadb", "cdfeb", "cdbaf"];
    const TEST_DATA: [&str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_display_get_1_wires() {
        let input: Vec<String> = TEST_UNIQUE_SIGNAL_PATTERNS
            .iter()
            .map(|s| s.to_string())
            .collect();

        let expected = [Wire::A, Wire::B].into_iter().collect();

        let result = Display::get_1_wires(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_display_get_4_wires() {
        let input: Vec<String> = TEST_UNIQUE_SIGNAL_PATTERNS
            .iter()
            .map(|s| s.to_string())
            .collect();

        let expected = [Wire::E, Wire::A, Wire::F, Wire::B].into_iter().collect();

        let result = Display::get_4_wires(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_display_get_7_wires() {
        let input: Vec<String> = TEST_UNIQUE_SIGNAL_PATTERNS
            .iter()
            .map(|s| s.to_string())
            .collect();

        let expected = [Wire::D, Wire::A, Wire::B].into_iter().collect();

        let result = Display::get_7_wires(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_display_get_8_wires() {
        let input: Vec<String> = TEST_UNIQUE_SIGNAL_PATTERNS
            .iter()
            .map(|s| s.to_string())
            .collect();

        let expected = [
            Wire::A,
            Wire::B,
            Wire::C,
            Wire::D,
            Wire::E,
            Wire::F,
            Wire::G,
        ]
        .into_iter()
        .collect();

        let result = Display::get_8_wires(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_display_map_wires_to_segments() {
        let input: Vec<String> = TEST_UNIQUE_SIGNAL_PATTERNS
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut display = Display::new();

        display.map_wires_to_segments(&input);

        let mut expected = HashMap::new();

        expected.insert(Wire::A, Segment::C);
        expected.insert(Wire::B, Segment::F);
        expected.insert(Wire::C, Segment::G);
        expected.insert(Wire::D, Segment::A);
        expected.insert(Wire::E, Segment::B);
        expected.insert(Wire::F, Segment::D);
        expected.insert(Wire::G, Segment::E);

        let result = display.wires_to_segments;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_display_get_display_number() {
        let input = "cdfbe";

        let mut wires_to_segments = HashMap::new();

        wires_to_segments.insert(Wire::A, Segment::C);
        wires_to_segments.insert(Wire::B, Segment::F);
        wires_to_segments.insert(Wire::C, Segment::G);
        wires_to_segments.insert(Wire::D, Segment::A);
        wires_to_segments.insert(Wire::E, Segment::B);
        wires_to_segments.insert(Wire::F, Segment::D);
        wires_to_segments.insert(Wire::G, Segment::E);

        let mut display = Display::new();

        display.wires_to_segments = wires_to_segments;

        let expected = 5;

        let result = display.get_display_number(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_displays_for() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = 61229;

        let result = get_displays_for(&input).iter().fold(0, |acc, x| acc + x);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_display_for() {
        let expected = 5353;

        let result = get_display_for(TEST_INPUT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_unique_signal_pattern_wires() {
        let input = "acedgfb";

        let mut expected = HashSet::new();

        expected.insert(Wire::A);
        expected.insert(Wire::C);
        expected.insert(Wire::E);
        expected.insert(Wire::D);
        expected.insert(Wire::G);
        expected.insert(Wire::F);
        expected.insert(Wire::B);

        let result = get_unique_signal_pattern_wires(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_number_of_1_4_7_or_8_displays() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = 26;

        let result = get_number_of_1_4_7_or_8_displays(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_patterns_and_displays() {
        let expected = (
            TEST_UNIQUE_SIGNAL_PATTERNS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            TEST_DISPLAY_PATTERNS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let result = get_patterns_and_displays(TEST_INPUT);

        assert_eq!(result, expected);
    }
}
