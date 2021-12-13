use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum CaveType {
    Small,
    Large,
}

#[derive(Debug, PartialEq)]
struct Cave {
    name: String,
    cave_type: CaveType,
    connected_cave_names: Vec<String>,
}

impl Cave {
    fn new(name: &str) -> Cave {
        let lowercase_name = name.to_ascii_lowercase();

        let cave_type = if lowercase_name == name {
            CaveType::Small
        } else {
            CaveType::Large
        };

        Cave {
            name: name.to_string(),
            cave_type,
            connected_cave_names: Vec::new(),
        }
    }

    fn connect_cave_name(&mut self, other_name: &str) {
        self.connected_cave_names.push(other_name.to_string());
    }

    fn is_start(&self) -> bool {
        self.name == "start"
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_small(&self) -> bool {
        self.cave_type == CaveType::Small
    }
}

#[derive(Debug, PartialEq)]
pub struct CaveSystem {
    cave_names_to_caves: HashMap<String, Cave>,
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        CaveSystem {
            cave_names_to_caves: HashMap::new(),
        }
    }

    pub fn new_from(cave_descriptions: &[String]) -> CaveSystem {
        let mut result = CaveSystem::new();

        for description in cave_descriptions {
            result.add_caves(description);
        }

        result
    }

    pub fn number_of_paths_to_end_visiting_small_caves_once(&self) -> u32 {
        let start_cave = self
            .cave_names_to_caves
            .get("start")
            .expect("Start cave is missing!");
        let mut small_caves_visited = HashSet::new();

        return self.number_of_paths_to_end_visiting_small_caves_once_from(
            start_cave,
            &mut small_caves_visited,
        );
    }

    pub fn number_of_paths_to_end_visiting_small_caves_once_maybe_twice(&self) -> u32 {
        let start_cave = self
            .cave_names_to_caves
            .get("start")
            .expect("Start cave is missing!");
        let mut small_caves_visited = HashSet::new();

        return self.number_of_paths_to_end_visiting_small_caves_once_maybe_twice_from(
            start_cave,
            &mut small_caves_visited,
            None,
        );
    }

    fn number_of_paths_to_end_visiting_small_caves_once_from(
        &self,
        cave: &Cave,
        small_caves_visited: &mut HashSet<String>,
    ) -> u32 {
        if cave.is_end() {
            return 1;
        }

        if cave.is_small() {
            small_caves_visited.insert(cave.name.clone());
        }

        let mut result = 0;

        for connected_cave_name in cave.connected_cave_names.iter() {
            if small_caves_visited.contains(connected_cave_name) {
                continue;
            }

            let connected_cave = self
                .cave_names_to_caves
                .get(connected_cave_name)
                .expect(&format!("Could not get cave: {}", connected_cave_name));

            result += self.number_of_paths_to_end_visiting_small_caves_once_from(
                connected_cave,
                small_caves_visited,
            );
        }

        small_caves_visited.remove(&cave.name);

        result
    }

    fn number_of_paths_to_end_visiting_small_caves_once_maybe_twice_from(
        &self,
        cave: &Cave,
        small_caves_visited: &mut HashSet<String>,
        small_cave_seen_twice: Option<String>,
    ) -> u32 {
        if cave.is_end() {
            return 1;
        }

        if cave.is_small() {
            small_caves_visited.insert(cave.name.clone());
        }

        let mut result = 0;

        for connected_cave_name in cave.connected_cave_names.iter() {
            let connected_cave = self
                .cave_names_to_caves
                .get(connected_cave_name)
                .expect(&format!("Could not get cave: {}", connected_cave_name));

            if small_caves_visited.contains(connected_cave_name) && small_cave_seen_twice.is_some()
            {
                continue;
            }

            if connected_cave.is_start() {
                continue;
            }

            let seen_twice = if small_cave_seen_twice.is_some() {
                small_cave_seen_twice.clone()
            } else if small_caves_visited.contains(connected_cave_name) {
                Some(connected_cave_name.clone())
            } else {
                None
            };

            result += self.number_of_paths_to_end_visiting_small_caves_once_maybe_twice_from(
                connected_cave,
                small_caves_visited,
                seen_twice,
            );
        }

        if let Some(twice_seen_cave_name) = small_cave_seen_twice {
            if twice_seen_cave_name != cave.name {
                small_caves_visited.remove(&cave.name);
            }
        } else {
            small_caves_visited.remove(&cave.name);
        }

        result
    }

    fn add_caves(&mut self, description: &str) {
        let (cave_1_name, cave_2_name) = description
            .split_once('-')
            .expect(&format!("Could not split: {}", description));

        let cave_1 = self
            .cave_names_to_caves
            .entry(cave_1_name.to_string())
            .or_insert(Cave::new(cave_1_name));

        cave_1.connect_cave_name(cave_2_name);

        let cave_2 = self
            .cave_names_to_caves
            .entry(cave_2_name.to_string())
            .or_insert(Cave::new(cave_2_name));

        cave_2.connect_cave_name(cave_1_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_SMALL: [&str; 7] =
        ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    const TEST_DATA_MEDIUM: [&str; 10] = [
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];
    const TEST_DATA_LARGE: [&str; 18] = [
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn test_cave_new() {
        let expected_small_cave = Cave {
            name: String::from("yr"),
            cave_type: CaveType::Small,
            connected_cave_names: Vec::new(),
        };
        let expected_large_cave = Cave {
            name: String::from("LD"),
            cave_type: CaveType::Large,
            connected_cave_names: Vec::new(),
        };

        let result_small_cave = Cave::new("yr");
        let result_large_cave = Cave::new("LD");

        assert_eq!(result_small_cave, expected_small_cave);
        assert_eq!(result_large_cave, expected_large_cave);
    }

    #[test]
    fn test_cave_system_add_caves() {
        let mut test_system = CaveSystem::new();

        test_system.add_caves(TEST_DATA_SMALL[0]);
        test_system.add_caves(TEST_DATA_SMALL[1]);

        let mut start_cave = Cave::new("start");
        let mut a_cave_large = Cave::new("A");
        let mut b_cave_small = Cave::new("b");

        start_cave.connect_cave_name(&a_cave_large.name);
        a_cave_large.connect_cave_name(&start_cave.name);

        start_cave.connect_cave_name(&b_cave_small.name);
        b_cave_small.connect_cave_name(&start_cave.name);

        let expected_caves = vec![
            (start_cave.name.clone(), start_cave),
            (a_cave_large.name.clone(), a_cave_large),
            (b_cave_small.name.clone(), b_cave_small),
        ]
        .into_iter()
        .collect();

        let expected_system = CaveSystem {
            cave_names_to_caves: expected_caves,
        };

        assert_eq!(test_system, expected_system);
    }

    #[test]
    fn test_cave_system_new_from() {
        let test_system = get_test_cave_system(&TEST_DATA_SMALL);

        let mut start_cave = Cave::new("start");
        let mut a_cave_large = Cave::new("A");
        let mut b_cave_small = Cave::new("b");
        let mut c_cave_small = Cave::new("c");
        let mut d_cave_small = Cave::new("d");
        let mut end_cave = Cave::new("end");

        start_cave.connect_cave_name(&a_cave_large.name);
        a_cave_large.connect_cave_name(&start_cave.name);

        start_cave.connect_cave_name(&b_cave_small.name);
        b_cave_small.connect_cave_name(&start_cave.name);

        a_cave_large.connect_cave_name(&c_cave_small.name);
        c_cave_small.connect_cave_name(&a_cave_large.name);

        a_cave_large.connect_cave_name(&b_cave_small.name);
        b_cave_small.connect_cave_name(&a_cave_large.name);

        b_cave_small.connect_cave_name(&d_cave_small.name);
        d_cave_small.connect_cave_name(&b_cave_small.name);

        a_cave_large.connect_cave_name(&end_cave.name);
        end_cave.connect_cave_name(&a_cave_large.name);

        b_cave_small.connect_cave_name(&end_cave.name);
        end_cave.connect_cave_name(&b_cave_small.name);

        let expected_caves = vec![
            (start_cave.name.clone(), start_cave),
            (a_cave_large.name.clone(), a_cave_large),
            (b_cave_small.name.clone(), b_cave_small),
            (c_cave_small.name.clone(), c_cave_small),
            (d_cave_small.name.clone(), d_cave_small),
            (end_cave.name.clone(), end_cave),
        ]
        .into_iter()
        .collect();

        let expected_system = CaveSystem {
            cave_names_to_caves: expected_caves,
        };

        assert_eq!(test_system, expected_system);
    }

    #[test]
    fn test_cave_system_number_of_paths_to_end_visiting_small_caves_once_from_zero() {
        let test_system = get_test_cave_system(&TEST_DATA_SMALL);

        let d_cave = test_system.cave_names_to_caves.get("d").unwrap();
        let mut seen_small_caves = vec![String::from("start"), String::from("b")]
            .into_iter()
            .collect();

        let expected = 0;
        let result = test_system
            .number_of_paths_to_end_visiting_small_caves_once_from(d_cave, &mut seen_small_caves);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cave_system_number_of_paths_to_end_visiting_small_caves_once_from_not_zero() {
        let test_system = get_test_cave_system(&TEST_DATA_SMALL);

        let b_cave = test_system.cave_names_to_caves.get("b").unwrap();
        let mut seen_small_caves = vec![String::from("start")].into_iter().collect();

        let expected = 3;
        let result = test_system
            .number_of_paths_to_end_visiting_small_caves_once_from(b_cave, &mut seen_small_caves);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cave_system_number_of_paths_to_end_visiting_small_caves_once_maybe_twice_from() {
        let test_system = get_test_cave_system(&TEST_DATA_SMALL);

        let b_cave = test_system.cave_names_to_caves.get("b").unwrap();
        let mut seen_small_caves = vec![String::from("start")].into_iter().collect();

        let expected = 12;
        let result = test_system.number_of_paths_to_end_visiting_small_caves_once_maybe_twice_from(
            b_cave,
            &mut seen_small_caves,
            None,
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cave_system_number_of_paths_to_end_visiting_small_caves_once() {
        let small_system = get_test_cave_system(&TEST_DATA_SMALL);
        let medium_system = get_test_cave_system(&TEST_DATA_MEDIUM);
        let large_system = get_test_cave_system(&TEST_DATA_LARGE);

        let expected_small = 10;
        let expected_medium = 19;
        let expected_large = 226;

        let result_small = small_system.number_of_paths_to_end_visiting_small_caves_once();
        let result_medium = medium_system.number_of_paths_to_end_visiting_small_caves_once();
        let result_large = large_system.number_of_paths_to_end_visiting_small_caves_once();

        assert_eq!(result_small, expected_small);
        assert_eq!(result_medium, expected_medium);
        assert_eq!(result_large, expected_large);
    }

    #[test]
    fn test_cave_system_number_of_paths_to_end_visiting_small_caves_once_maybe_twice() {
        let small_system = get_test_cave_system(&TEST_DATA_SMALL);
        let medium_system = get_test_cave_system(&TEST_DATA_MEDIUM);
        let large_system = get_test_cave_system(&TEST_DATA_LARGE);

        let expected_small = 36;
        let expected_medium = 103;
        let expected_large = 3509;

        let result_small =
            small_system.number_of_paths_to_end_visiting_small_caves_once_maybe_twice();
        let result_medium =
            medium_system.number_of_paths_to_end_visiting_small_caves_once_maybe_twice();
        let result_large =
            large_system.number_of_paths_to_end_visiting_small_caves_once_maybe_twice();

        assert_eq!(result_small, expected_small);
        assert_eq!(result_medium, expected_medium);
        assert_eq!(result_large, expected_large);
    }
    fn get_test_cave_system(data: &[&str]) -> CaveSystem {
        let input: Vec<String> = data.iter().map(|s| s.to_string()).collect();

        CaveSystem::new_from(&input)
    }
}
