use super::equal_to_operation_packet::EqualToOperationPacket;
use super::greater_than_operation_packet::GreaterThanOperationPacket;
use super::less_than_operation_packet::LessThanOperationPacket;
use super::literal_packet::LiteralValuePacket;
use super::maximum_operation_packet::MaximumOperationPacket;
use super::minimum_operation_packet::MinimumOperationPacket;
use super::product_operation_packet::ProductOperationPacket;
use super::sum_operation_packet::SumOperationPacket;

#[derive(Debug, PartialEq)]
pub enum Packet {
    SumOperation(SumOperationPacket),
    ProductOperation(ProductOperationPacket),
    MinimumOperation(MinimumOperationPacket),
    MaximumOperation(MaximumOperationPacket),
    Literal(LiteralValuePacket),
    GreaterThanOperation(GreaterThanOperationPacket),
    LessThanOperation(LessThanOperationPacket),
    EqualToOperation(EqualToOperationPacket),
}

impl Packet {
    pub fn get_total_version_sum(&self) -> usize {
        match self {
            Packet::SumOperation(packet) => packet.get_total_version_sum(),
            Packet::ProductOperation(packet) => packet.get_total_version_sum(),
            Packet::MinimumOperation(packet) => packet.get_total_version_sum(),
            Packet::MaximumOperation(packet) => packet.get_total_version_sum(),
            Packet::Literal(packet) => packet.get_version() as usize,
            Packet::GreaterThanOperation(packet) => packet.get_total_version_sum(),
            Packet::LessThanOperation(packet) => packet.get_total_version_sum(),
            Packet::EqualToOperation(packet) => packet.get_total_version_sum(),
        }
    }

    pub fn get_value(&self) -> usize {
        match self {
            Packet::SumOperation(packet) => packet.get_value(),
            Packet::ProductOperation(packet) => packet.get_value(),
            Packet::MinimumOperation(packet) => packet.get_value(),
            Packet::MaximumOperation(packet) => packet.get_value(),
            Packet::Literal(packet) => packet.get_value(),
            Packet::GreaterThanOperation(packet) => packet.get_value(),
            Packet::LessThanOperation(packet) => packet.get_value(),
            Packet::EqualToOperation(packet) => packet.get_value(),
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Packet::SumOperation(packet) => packet.get_size(),
            Packet::ProductOperation(packet) => packet.get_size(),
            Packet::MinimumOperation(packet) => packet.get_size(),
            Packet::MaximumOperation(packet) => packet.get_size(),
            Packet::Literal(packet) => packet.get_size(),
            Packet::GreaterThanOperation(packet) => packet.get_size(),
            Packet::LessThanOperation(packet) => packet.get_size(),
            Packet::EqualToOperation(packet) => packet.get_size(),
        }
    }
}

impl From<&str> for Packet {
    fn from(input: &str) -> Packet {
        let type_id =
            u8::from_str_radix(&input[3..6], 2).expect(&format!("Couldn't parse type: {}", input));

        match type_id {
            0 => Packet::SumOperation(SumOperationPacket::from(input)),
            1 => Packet::ProductOperation(ProductOperationPacket::from(input)),
            2 => Packet::MinimumOperation(MinimumOperationPacket::from(input)),
            3 => Packet::MaximumOperation(MaximumOperationPacket::from(input)),
            4 => Packet::Literal(LiteralValuePacket::from(input)),
            5 => Packet::GreaterThanOperation(GreaterThanOperationPacket::from(input)),
            6 => Packet::LessThanOperation(LessThanOperationPacket::from(input)),
            7 => Packet::EqualToOperation(EqualToOperationPacket::from(input)),
            _ => unimplemented!("Packet Type Id not implemented: {}", type_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 3] = [
        "110100101111111000101000",
        "00111000000000000110111101000101001010010001001000000000",
        "11101110000000001101010000001100100000100011000001100000",
    ];

    const TEST_OPERATION_DATA: [&str; 8] = [
        "1100001000000000101101000000101010000010",
        "000001000000000001011010110000110011100010010000",
        "10001000000000001000011011000011111010001000000100010010",
        "11001110000000001100010000111101100010000001000100100000",
        "1111011000000000101111000010110110001111",
        "110110000000000001011010110000101010100011110000",
        "100111000000000001011010110000101111100011110000",
        "10011100000000010100000100001000000000100101000000110010000011110001100000000010000100000100101000001000",
    ];

    #[test]
    fn test_packet_literal_from() {
        let expected = Packet::Literal(LiteralValuePacket::from(TEST_DATA[0]));

        let result = Packet::from(TEST_DATA[0]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_operation_from() {
        let expected = Packet::LessThanOperation(LessThanOperationPacket::from(TEST_DATA[1]));

        let result = Packet::from(TEST_DATA[1]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_get_total_version_sum() {
        let expected = vec![6, 9, 14];

        let result: Vec<usize> = TEST_DATA
            .iter()
            .map(|s| Packet::from(*s).get_total_version_sum())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_packet_get_value() {
        let expected = vec![3, 54, 7, 9, 0, 1, 0, 1];

        let result: Vec<usize> = TEST_OPERATION_DATA
            .iter()
            .map(|s| Packet::from(*s).get_value())
            .collect();

        assert_eq!(result, expected);
    }
}
