#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LengthId {
    TotalSizeOfSubpackets(usize),
    NumberOfSubpackets(usize),
}

impl LengthId {
    pub fn get_size(&self) -> usize {
        match self {
            LengthId::TotalSizeOfSubpackets(_) => 16,
            LengthId::NumberOfSubpackets(_) => 12,
        }
    }
}

impl From<&str> for LengthId {
    fn from(input: &str) -> LengthId {
        let indication = input
            .chars()
            .nth(6)
            .expect(&format!("Couldn't parse length id: {}", input));

        let possible_total_size = usize::from_str_radix(&input[7..22], 2)
            .expect(&format!("Couldn't parse total size: {}", input));

        let possible_total_number = usize::from_str_radix(&input[7..18], 2)
            .expect(&format!("Couldn't parse total number: {}", input));

        match indication {
            '0' => LengthId::TotalSizeOfSubpackets(possible_total_size),
            '1' => LengthId::NumberOfSubpackets(possible_total_number),
            _ => panic!("Unknown length id: {}", input),
        }
    }
}
