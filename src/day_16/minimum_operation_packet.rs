use super::operation_packet::OperationPacket;

#[derive(Debug, PartialEq)]
pub struct MinimumOperationPacket {
    operation_packet: OperationPacket,
}

impl MinimumOperationPacket {
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
            .min()
            .expect("Couldn't find the minimum!")
    }
}

impl From<&str> for MinimumOperationPacket {
    fn from(input: &str) -> MinimumOperationPacket {
        let operation = OperationPacket::from(input);

        MinimumOperationPacket {
            operation_packet: operation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "10001000000000001000011011000011111010001000000100010010";

    #[test]
    fn get_value() {
        let expected = 7;

        let result = MinimumOperationPacket::from(TEST_DATA).get_value();

        assert_eq!(result, expected);
    }
}
