#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PacketHeader {
    version: u8,
    type_id: u8,
}

impl PacketHeader {
    pub fn get_version(&self) -> u8 {
        self.version
    }
}

impl From<&str> for PacketHeader {
    fn from(input: &str) -> PacketHeader {
        let version = u8::from_str_radix(&input[0..3], 2)
            .expect(&format!("Couldn't parse version: {}", input));

        let type_id = u8::from_str_radix(&input[3..6], 2)
            .expect(&format!("Couldn't parse type id: {}", input));

        PacketHeader { version, type_id }
    }
}
