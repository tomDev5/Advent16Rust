use super::{literal::parse_literal, operator::parse_operator, return_info::ParseReturnInfo};
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

pub fn parse_packet_from_base(position: (&[u8], usize)) -> Result<ParseReturnInfo, DekuError> {
    let (position, base) = PacketBase::from_bytes(position)?;

    let return_info = match base.packet_type {
        PacketType::Literal => parse_literal(position)?,
        operator_code => parse_operator(operator_code, position)?,
    };

    Ok(ParseReturnInfo {
        position: return_info.position,
        bits_read: 6 + return_info.bits_read,
        packets_read: return_info.packets_read,
        value: return_info.value,
    })
}
