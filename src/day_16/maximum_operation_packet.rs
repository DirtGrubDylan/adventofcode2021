use super::operation_packet::OperationPacket;

#[derive(Debug, PartialEq)]
pub struct MaximumOperationPacket {
    operation_packet: OperationPacket,
}

impl MaximumOperationPacket {
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
            .max()
            .expect("Cound't find maximum!")
    }
}

impl From<&str> for MaximumOperationPacket {
    fn from(input: &str) -> MaximumOperationPacket {
        let operation = OperationPacket::from(input);

        MaximumOperationPacket {
            operation_packet: operation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "11001110000000001100010000111101100010000001000100100000";

    #[test]
    fn get_value() {
        let expected = 9;

        let result = MaximumOperationPacket::from(TEST_DATA).get_value();

        assert_eq!(result, expected);
    }
}
