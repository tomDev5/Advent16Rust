use core::panic;
use deku::DekuContainerRead;

use crate::{
    protocols::base::parse_packet_from_base,
    protocols::{
        bit_len_operator::BitLenOperator, packet_num_operator::PacketNumOperator,
        return_info::ParseReturnInfo,
    },
};

use super::base::PacketType;

pub fn parse_operator(operator: PacketType, position: (&[u8], usize)) -> ParseReturnInfo {
    let bit_len_operator = BitLenOperator::from_bytes(position);
    let packet_count_operator = PacketNumOperator::from_bytes(position);

    if let Ok((position, bit_len_operator)) = bit_len_operator {
        let condition = move |return_info: &ParseReturnInfo| {
            return_info.bits_read < bit_len_operator.bits as usize
        };

        perform_operator_action(operator, 16, condition, position)
    } else if let Ok((position, packet_count_operator)) = packet_count_operator {
        let condition = move |return_info: &ParseReturnInfo| {
            return_info.packets_read < packet_count_operator.packets as usize
        };

        perform_operator_action(operator, 12, condition, position)
    } else {
        panic!("WTF");
    }
}

fn perform_operator_action(
    operator: PacketType,
    header_len: usize,
    condition: impl Fn(&ParseReturnInfo) -> bool,
    position: (&[u8], usize),
) -> ParseReturnInfo {
    let return_info = match operator {
        PacketType::Sum => parse_while_calculative(position, i128::checked_add, condition),
        PacketType::Product => parse_while_calculative(position, i128::checked_mul, condition),
        PacketType::Minimum => parse_subpackets(position, i128::min, condition),
        PacketType::Maximum => parse_subpackets(position, i128::max, condition),
        PacketType::GreaterThan => parse_subpackets(position, |lhs, rhs| lhs > rhs, condition),
        PacketType::LessThan => parse_subpackets(position, |lhs, rhs| lhs < rhs, condition),
        PacketType::Equal => parse_subpackets(position, |lhs, rhs| lhs == rhs, condition),
        other => panic!("WTF2: {:?}", other),
    };

    ParseReturnInfo {
        position: return_info.position,
        bits_read: header_len + return_info.bits_read,
        packets_read: return_info.packets_read,
        value: return_info.value,
    }
}

fn parse_while_calculative<A, F>(
    position: (&[u8], usize),
    action: A,
    condition: F,
) -> ParseReturnInfo
where
    A: Fn(i128, i128) -> Option<i128>,
    F: Fn(&ParseReturnInfo) -> bool,
{
    let mut return_info = ParseReturnInfo {
        position,
        bits_read: 0,
        packets_read: 0,
        value: 0,
    };
    while condition(&return_info) {
        let returned = parse_packet_from_base(return_info.position);
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += 1;
        return_info.value = action(return_info.value, returned.value).unwrap();
    }
    return_info
}

fn parse_subpackets<A, R, F>(position: (&[u8], usize), action: A, condition: F) -> ParseReturnInfo
where
    F: Fn(&ParseReturnInfo) -> bool,
    A: Fn(i128, i128) -> R,
    i128: From<R>,
{
    let mut return_info = ParseReturnInfo {
        position,
        bits_read: 0,
        packets_read: 0,
        value: 0,
    };
    while condition(&return_info) {
        let returned = parse_packet_from_base(return_info.position);
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += 1;
        return_info.value = i128::from(action(return_info.value, returned.value));
    }
    return_info
}
