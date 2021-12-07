use std::collections::HashMap;

const BIRTH_COOLDOWN: i32 = 2;
const BIRTH_CYCLE: i32 = 7;
const BIRTH_CYCLE_STEP: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanternFish {
    day_born: i32,
}

impl LanternFish {
    pub fn new(days_until_spawn: i32) -> LanternFish {
        LanternFish::spawn(days_until_spawn - 8)
    }

    pub fn spawn(day_born: i32) -> LanternFish {
        LanternFish { day_born }
    }

    pub fn get_number_of_relatives_created_by_day(
        &self,
        end_day: i32,
        born_day_to_number_of_spawn: &mut HashMap<i32, u64>,
    ) -> u64 {
        if born_day_to_number_of_spawn.contains_key(&self.day_born) {
            return *born_day_to_number_of_spawn.get(&self.day_born).unwrap();
        }

        let number_of_spawn =
            self.get_number_of_spawn_from_birth(end_day, born_day_to_number_of_spawn);

        born_day_to_number_of_spawn.insert(self.day_born, number_of_spawn);

        number_of_spawn
    }

    fn get_number_of_spawn_from_birth(
        &self,
        end_day: i32,
        born_day_to_number_of_spawn: &mut HashMap<i32, u64>,
    ) -> u64 {
        let mut result = 0;

        let spawn_day_start = self.day_born + BIRTH_COOLDOWN + BIRTH_CYCLE;

        for spawn_day in (spawn_day_start..=end_day).step_by(BIRTH_CYCLE_STEP) {
            let number_of_spawn = LanternFish::spawn(spawn_day)
                .get_number_of_relatives_created_by_day(end_day, born_day_to_number_of_spawn);

            result += 1 + number_of_spawn;
        }

        result
    }
}

pub fn get_lanternfish_population_created_in_days(initial_fish: &[LanternFish], days: i32) -> u64 {
    let mut result = 0;
    let mut born_day_to_number_of_spawn: HashMap<i32, u64> = HashMap::new();

    for fish in initial_fish.iter() {
        let spawn = if born_day_to_number_of_spawn.contains_key(&fish.day_born) {
            *born_day_to_number_of_spawn.get(&fish.day_born).unwrap()
        } else {
            let result =
                fish.get_number_of_relatives_created_by_day(days, &mut born_day_to_number_of_spawn);

            born_day_to_number_of_spawn.insert(fish.day_born, result);

            result
        };

        result += 1 + spawn;
    }

    result
}

pub fn get_initial_fish(input: &str) -> Vec<LanternFish> {
    input
        .split(',')
        .map(|split| {
            split
                .parse()
                .expect(&format!("Could not parse u64: {}", split))
        })
        .map(|days_until_spawn| LanternFish::new(days_until_spawn))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_STR: &str = "3,4,3,1,2";
    const TEST_DATA: [i32; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test_lanternfish_new() {
        let expected = LanternFish { day_born: -5 };

        let result = LanternFish::new(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lanternfish_get_number_of_relatives_created_by_day_zero() {
        let mut hashmap = HashMap::new();

        let test_fish_0 = LanternFish { day_born: 10 };
        let test_fish_1 = LanternFish { day_born: 18 };
        let test_fish_2 = LanternFish { day_born: 19 };

        let expected = 0;

        let result_0 = test_fish_0.get_number_of_relatives_created_by_day(18, &mut hashmap);
        let result_1 = test_fish_1.get_number_of_relatives_created_by_day(18, &mut hashmap);
        let result_2 = test_fish_2.get_number_of_relatives_created_by_day(18, &mut hashmap);

        assert_eq!(result_0, expected);
        assert_eq!(result_1, expected);
        assert_eq!(result_2, expected);
    }

    #[test]
    fn test_lanternfish_get_number_of_relatives_created_by_day_non_zero() {
        let mut hashmap = HashMap::new();

        let test_fish_0 = LanternFish::spawn(-5);
        let test_fish_1 = LanternFish::spawn(9);
        let test_fish_2 = LanternFish::spawn(-7);

        let expected_0 = 4;
        let expected_1 = 1;
        let expected_2 = 6;

        let result_0 = test_fish_0.get_number_of_relatives_created_by_day(18, &mut hashmap);
        let result_1 = test_fish_1.get_number_of_relatives_created_by_day(18, &mut hashmap);
        let result_2 = test_fish_2.get_number_of_relatives_created_by_day(18, &mut hashmap);

        assert_eq!(result_0, expected_0);
        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_lanternfish_population_created_in_days() {
        let initial_fish: Vec<LanternFish> = TEST_DATA
            .iter()
            .map(|&days_until_spawn| LanternFish::new(days_until_spawn))
            .collect();

        let expected_0 = 26;
        let expected_1 = 5934;
        let expected_2 = 26984457539;

        let result_0 = get_lanternfish_population_created_in_days(&initial_fish, 18);
        let result_1 = get_lanternfish_population_created_in_days(&initial_fish, 80);
        let result_2 = get_lanternfish_population_created_in_days(&initial_fish, 256);

        assert_eq!(result_0, expected_0);
        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_initial_fish() {
        let expected: Vec<LanternFish> = TEST_DATA
            .iter()
            .map(|&days_until_spawn| LanternFish::new(days_until_spawn))
            .collect();

        let result = get_initial_fish(TEST_DATA_STR);

        assert_eq!(result, expected);
    }
}
