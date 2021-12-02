use crate::util::point_3d::Point3d;

pub fn get_final_location(input: &[String]) -> Point3d<i32> {
    input
        .iter()
        .map(|instruction| convert_instruction_to_relative_location(instruction))
        .fold(Point3d::new(0, 0, 0), |running_sum, relative_location| {
            next_location(&running_sum, &relative_location)
        })
}

fn next_location(current_location: &Point3d<i32>, location_change: &Point3d<i32>) -> Point3d<i32> {
    let x_position = current_location.x + location_change.x;
    let aim = current_location.y + location_change.y;
    let depth = current_location.z + location_change.x * current_location.y;

    Point3d::new(x_position, aim, depth)
}

fn convert_instruction_to_relative_location(instruction: &str) -> Point3d<i32> {
    let (direction, distance_str) = instruction
        .split_once(' ')
        .expect(&format!("Cannot split instuction: {}", instruction));

    let distance: i32 = distance_str
        .parse()
        .expect(&format!("Cannot parse distance: {}", distance_str));

    match direction {
        "forward" => Point3d::new(distance, 0, 0),
        "down" => Point3d::new(0, -distance, 0),
        "up" => Point3d::new(0, distance, 0),
        _ => panic!("Unexpected direction: {}", direction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::point_3d::Point3d;

    const TEST_DATA: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_get_final_location() {
        let input: Vec<String> = TEST_DATA.iter().map(|s| s.to_string()).collect();

        let expected = Point3d::new(15, -10, -60);

        let result = get_final_location(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_location() {
        let current_location = Point3d::new(13, -10, -40);
        let location_change = Point3d::new(2, 0, 0);

        let expected = Point3d::new(15, -10, -60);

        let result = next_location(&current_location, &location_change);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_convert_instruction_to_relative_location() {
        let expected_forward = Point3d::new(5, 0, 0);
        let expected_down = Point3d::new(0, -5, 0);
        let expected_up = Point3d::new(0, 3, 0);

        let result_forward = convert_instruction_to_relative_location(TEST_DATA[0]);
        let result_down = convert_instruction_to_relative_location(TEST_DATA[1]);
        let result_up = convert_instruction_to_relative_location(TEST_DATA[3]);

        assert_eq!(result_forward, expected_forward);
        assert_eq!(result_down, expected_down);
        assert_eq!(result_up, expected_up);
    }
}
