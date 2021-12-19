use super::operation_packet::OperationPacket;

#[derive(Debug, PartialEq)]
pub struct ProductOperationPacket {
    operation_packet: OperationPacket,
}

impl ProductOperationPacket {
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
            .fold(1, |acc, value| acc * value)
    }
}

impl From<&str> for ProductOperationPacket {
    fn from(input: &str) -> ProductOperationPacket {
        let operation = OperationPacket::from(input);

        ProductOperationPacket {
            operation_packet: operation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "000001000000000001011010110000110011100010010000";

    #[test]
    fn get_value() {
        let expected = 54;

        let result = ProductOperationPacket::from(TEST_DATA).get_value();

        assert_eq!(result, expected);
    }
}
