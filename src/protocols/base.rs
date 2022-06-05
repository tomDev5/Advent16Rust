use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead)]
#[deku(
    type = "u8",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    bits = "3"
)]
pub enum PacketType {
    Literal = 4,
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "big")]
pub struct PacketBase {
    #[deku(bits = "3")]
    pub version: u8,
    pub packet_type: PacketType,
}
