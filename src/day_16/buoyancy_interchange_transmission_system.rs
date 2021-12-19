use super::hex_converter::hex_str_to_binary_string;
use super::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct BuoyancyInterchangeTransmissionSystem {
    pub outer_packet: Packet,
}

impl BuoyancyInterchangeTransmissionSystem {
    pub fn new(input: &str) -> BuoyancyInterchangeTransmissionSystem {
        BuoyancyInterchangeTransmissionSystem {
            outer_packet: Packet::from(hex_str_to_binary_string(input).as_str()),
        }
    }

    pub fn get_total_version_sum(&self) -> usize {
        self.outer_packet.get_total_version_sum()
    }

    pub fn get_value(&self) -> usize {
        self.outer_packet.get_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 4] = [
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];

    const TEST_DATA_LARGE: [&str; 8] = [
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
    ];

    #[test]
    fn test_get_total_version_sum() {
        let expected = vec![16, 12, 23, 31];

        let result: Vec<usize> = TEST_DATA
            .iter()
            .map(|s| BuoyancyInterchangeTransmissionSystem::new(s).get_total_version_sum())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_value() {
        let expected = vec![3, 54, 7, 9, 1, 0, 0, 1];

        let result: Vec<usize> = TEST_DATA_LARGE
            .iter()
            .map(|s| BuoyancyInterchangeTransmissionSystem::new(s).get_value())
            .collect();

        assert_eq!(result, expected);
    }
}
