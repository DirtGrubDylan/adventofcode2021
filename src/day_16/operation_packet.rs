use super::length_id::LengthId;
use super::packet::Packet;
use super::packet_header::PacketHeader;

#[derive(Debug, PartialEq)]
pub struct OperationPacket {
    header: PacketHeader,
    length_id: LengthId,
    subpackets: Vec<Packet>,
    size: usize,
}

impl OperationPacket {
    pub fn get_version(&self) -> u8 {
        self.header.get_version()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_total_version_sum(&self) -> usize {
        (self.get_version() as usize)
            + self
                .subpackets
                .iter()
                .fold(0, |acc, subpacket| acc + subpacket.get_total_version_sum())
    }

    pub fn get_subpackets(&self) -> &[Packet] {
        &self.subpackets
    }

    pub fn build_subpackets(input: &str) -> Vec<Packet> {
        let length_id = LengthId::from(input);

        let starting_index = 6 + length_id.get_size();

        match length_id {
            LengthId::TotalSizeOfSubpackets(total_size) => {
                Self::build_subpackets_by_total_size(total_size, &input[starting_index..])
            }
            LengthId::NumberOfSubpackets(count) => {
                Self::build_subpackets_by_counting(count, &input[starting_index..])
            }
        }
    }

    fn build_subpackets_by_total_size(total_size: usize, input: &str) -> Vec<Packet> {
        let mut result = Vec::new();
        let mut running_size = 0;

        while running_size != total_size {
            let subpacket = Packet::from(&input[running_size..]);

            running_size += subpacket.get_size();

            result.push(subpacket);
        }

        result
    }

    fn build_subpackets_by_counting(count: usize, input: &str) -> Vec<Packet> {
        let mut result = Vec::new();
        let mut running_count = 0;
        let mut running_size = 0;

        while running_count != count {
            let subpacket = Packet::from(&input[running_size..]);

            running_count += 1;
            running_size += subpacket.get_size();

            result.push(subpacket);
        }

        result
    }
}

impl From<&str> for OperationPacket {
    fn from(input: &str) -> OperationPacket {
        let header = PacketHeader::from(input);

        let length_id = LengthId::from(input);
        let mut size = 6;

        size += length_id.get_size();

        let subpackets = OperationPacket::build_subpackets(input);

        size += subpackets
            .iter()
            .map(|subpacket| subpacket.get_size())
            .sum::<usize>();

        OperationPacket {
            header,
            length_id,
            subpackets,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_16::literal_packet::LiteralValuePacket;

    const TEST_DATA: [&str; 2] = [
        "00111000000000000110111101000101001010010001001000000000",
        "11101110000000001101010000001100100000100011000001100000",
    ];

    #[test]
    fn test_operation_build_subpackets() {
        let expected_subpacket_1 = Packet::Literal(LiteralValuePacket::from("11010001010"));
        let expected_subpacket_2 = Packet::Literal(LiteralValuePacket::from("0101001000100100"));
        let expected = vec![expected_subpacket_1, expected_subpacket_2];

        let result = OperationPacket::build_subpackets(TEST_DATA[0]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_operation_size_from() {
        let expected_subpacket_1 = Packet::Literal(LiteralValuePacket::from("11010001010"));
        let expected_subpacket_2 = Packet::Literal(LiteralValuePacket::from("0101001000100100"));
        let expected = OperationPacket {
            header: PacketHeader::from(TEST_DATA[0]),
            length_id: LengthId::from(TEST_DATA[0]),
            subpackets: vec![expected_subpacket_1, expected_subpacket_2],
            size: 49,
        };

        let result = OperationPacket::from(TEST_DATA[0]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_operation_count_from() {
        let expected_subpacket_1 = Packet::Literal(LiteralValuePacket::from("01010000001"));
        let expected_subpacket_2 = Packet::Literal(LiteralValuePacket::from("10010000010"));
        let expected_subpacket_3 = Packet::Literal(LiteralValuePacket::from("00110000011"));
        let expected = OperationPacket {
            header: PacketHeader::from(TEST_DATA[1]),
            length_id: LengthId::from(TEST_DATA[1]),
            subpackets: vec![
                expected_subpacket_1,
                expected_subpacket_2,
                expected_subpacket_3,
            ],
            size: 51,
        };

        let result = OperationPacket::from(TEST_DATA[1]);

        assert_eq!(result, expected);
    }
}
