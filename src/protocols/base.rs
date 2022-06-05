use super::{
    literal::parse_literal,
    operator::{parse_operator, Operator},
    return_info::ParseReturnInfo,
};
use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(
    type = "u8",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    bits = "3"
)]
pub enum PacketType {
    #[deku(id = "4")]
    Literal,
    #[deku(id_pat = "_")]
    Operator(Operator),
}

#[derive(Debug, DekuRead)]
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
        PacketType::Operator(operator) => parse_operator(operator, position)?,
    };

    Ok(ParseReturnInfo {
        bits_read: 6 + return_info.bits_read,
        ..return_info
    })
}
