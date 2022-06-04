use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct PacketBase {
    #[deku(bits = "3")]
    pub version: u8,
    #[deku(bits = "3")]
    pub next_proto: u8,
}
