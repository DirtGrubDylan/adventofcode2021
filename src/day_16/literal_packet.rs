use super::packet_header::PacketHeader;

#[derive(Debug, PartialEq)]
pub struct LiteralValuePacket {
    header: PacketHeader,
    value: usize,
    size: usize,
}

impl LiteralValuePacket {
    pub fn get_version(&self) -> u8 {
        self.header.get_version()
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl From<&str> for LiteralValuePacket {
    fn from(input: &str) -> LiteralValuePacket {
        let header = PacketHeader::from(input);

        let mut size = 6;
        let mut value = 0;

        let mut stop = false;

        for (index, bit) in input.chars().skip(6).enumerate() {
            let bit_is_zero = bit == '0';

            if (index % 5 == 0) && stop {
                break;
            } else if index % 5 == 0 {
                stop |= bit_is_zero;
            } else {
                let temp_value = if bit_is_zero { 0 } else { 1 };

                value = value * 2 + temp_value;
            }

            size += 1;
        }

        LiteralValuePacket {
            header,
            value,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "110100101111111000101000";

    #[test]
    fn test_literal_value_from() {
        let expected = LiteralValuePacket {
            header: PacketHeader::from("110100"),
            value: 2021,
            size: 21,
        };

        let result = LiteralValuePacket::from(TEST_DATA);

        assert_eq!(result, expected);
    }
}
