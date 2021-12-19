use super::operation_packet::OperationPacket;

#[derive(Debug, PartialEq)]
pub struct SumOperationPacket {
    operation_packet: OperationPacket,
}

impl SumOperationPacket {
    pub fn get_size(&self) -> usize {
        self.operation_packet.get_size()
    }

    pub fn get_total_version_sum(&self) -> usize {
        self.operation_packet.get_total_version_sum()
    }

    pub fn get_value(&self) -> usize {
        self.operation_packet
            .get_subpackets()
            .iter()
            .map(|subpacket| subpacket.get_value())
            .sum::<usize>()
    }
}

impl From<&str> for SumOperationPacket {
    fn from(input: &str) -> SumOperationPacket {
        let operation = OperationPacket::from(input);

        SumOperationPacket {
            operation_packet: operation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1100001000000000101101000000101010000010";

    #[test]
    fn get_value() {
        let expected = 3;

        let result = SumOperationPacket::from(TEST_DATA).get_value();

        assert_eq!(result, expected);
    }
}
