use super::operation_packet::OperationPacket;

#[derive(Debug, PartialEq)]
pub struct GreaterThanOperationPacket {
    operation_packet: OperationPacket,
}

impl GreaterThanOperationPacket {
    pub fn get_size(&self) -> usize {
        self.operation_packet.get_size()
    }

    pub fn get_total_version_sum(&self) -> usize {
        self.operation_packet.get_total_version_sum()
    }

    pub fn get_value(&self) -> usize {
        if self.operation_packet.get_subpackets().len() != 2 {
            panic!(
                "Greater than operation does not contain exactly 2 subpackets: {:?}",
                self.operation_packet
            );
        }

        let subpacket_1_value = self
            .operation_packet
            .get_subpackets()
            .get(0)
            .unwrap()
            .get_value();
        let subpacket_2_value = self
            .operation_packet
            .get_subpackets()
            .get(1)
            .unwrap()
            .get_value();

        (subpacket_1_value > subpacket_2_value) as usize
    }
}

impl From<&str> for GreaterThanOperationPacket {
    fn from(input: &str) -> GreaterThanOperationPacket {
        let operation = OperationPacket::from(input);

        GreaterThanOperationPacket {
            operation_packet: operation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1111011000000000101111000010110110001111";

    #[test]
    fn get_value() {
        let expected = 0;

        let result = GreaterThanOperationPacket::from(TEST_DATA).get_value();

        assert_eq!(result, expected);
    }
}
