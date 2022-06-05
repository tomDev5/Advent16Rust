use super::base::PacketType;
use crate::{protocols::base::parse_packet_from_base, protocols::return_info::ParseReturnInfo};
use core::panic;
use deku::DekuContainerRead;
use deku::DekuEnumExt;
use deku::{DekuError, DekuRead};

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "big")]
#[deku(type = "u16", bits = "1")]
enum LengthOperator {
    #[deku(id = "0")]
    BitLenOperator(#[deku(bits = "15")] u16),
    #[deku(id = "1")]
    PacketNumOperator(#[deku(bits = "11")] u16),
}

pub fn parse_operator(
    operator: PacketType,
    position: (&[u8], usize),
) -> Result<ParseReturnInfo, DekuError> {
    let (position, length_operator) = LengthOperator::from_bytes(position)?;

    match length_operator {
        LengthOperator::BitLenOperator(bits) => {
            let condition =
                move |return_info: &ParseReturnInfo| return_info.bits_read < bits as usize;

            perform_operator_action(operator, 16, condition, position)
        }
        LengthOperator::PacketNumOperator(packets) => {
            let condition =
                move |return_info: &ParseReturnInfo| return_info.packets_read < packets as usize;

            perform_operator_action(operator, 12, condition, position)
        }
    }
}

fn perform_operator_action(
    operator: PacketType,
    header_len: usize,
    condition: impl Fn(&ParseReturnInfo) -> bool,
    position: (&[u8], usize),
) -> Result<ParseReturnInfo, DekuError> {
    let return_info = match operator {
        PacketType::Sum => parse_while_calculative(position, i128::checked_add, condition)?,
        PacketType::Product => parse_while_calculative(position, i128::checked_mul, condition)?,
        PacketType::Minimum => parse_subpackets(position, i128::min, condition)?,
        PacketType::Maximum => parse_subpackets(position, i128::max, condition)?,
        PacketType::GreaterThan => parse_subpackets(position, |lhs, rhs| lhs > rhs, condition)?,
        PacketType::LessThan => parse_subpackets(position, |lhs, rhs| lhs < rhs, condition)?,
        PacketType::Equal => parse_subpackets(position, |lhs, rhs| lhs == rhs, condition)?,
        other => panic!("WTF2: {:?}", other),
    };

    Ok(ParseReturnInfo {
        position: return_info.position,
        bits_read: header_len + return_info.bits_read,
        packets_read: return_info.packets_read,
        value: return_info.value,
    })
}

fn parse_while_calculative<A, F>(
    position: (&[u8], usize),
    action: A,
    condition: F,
) -> Result<ParseReturnInfo, DekuError>
where
    A: Fn(i128, i128) -> Option<i128>,
    F: Fn(&ParseReturnInfo) -> bool,
{
    let mut return_info = parse_packet_from_base(position)?;
    while condition(&return_info) {
        let returned = parse_packet_from_base(return_info.position)?;
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += 1;

        if let Some(action) = action(return_info.value, returned.value) {
            return_info.value = action;
        } else {
            return Err(DekuError::Unexpected(
                "Unsafe operation returned none".to_string(),
            ));
        }
    }
    Ok(return_info)
}

fn parse_subpackets<A, R, F>(
    position: (&[u8], usize),
    action: A,
    condition: F,
) -> Result<ParseReturnInfo, DekuError>
where
    F: Fn(&ParseReturnInfo) -> bool,
    A: Fn(i128, i128) -> R,
    i128: From<R>,
{
    let mut return_info = parse_packet_from_base(position)?;
    while condition(&return_info) {
        let returned = parse_packet_from_base(return_info.position)?;
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += 1;
        return_info.value = i128::from(action(return_info.value, returned.value));
    }
    Ok(return_info)
}
