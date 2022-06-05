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

fn parse_while_calculative<C: Fn(i128, i128) -> Option<i128>, F: Fn(&ParseReturnInfo) -> bool>(
    position: (&[u8], usize),
    calculation: C,
    condition: F,
) -> ParseReturnInfo {
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
        return_info.value = calculation(return_info.value, returned.value).unwrap();
    }
    return_info
}

fn parse_while_selective<S: Fn(i128, i128) -> i128, F: Fn(&ParseReturnInfo) -> bool>(
    position: (&[u8], usize),
    selection: S,
    condition: F,
) -> ParseReturnInfo {
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
        return_info.value = selection(return_info.value, returned.value);
    }
    return_info
}

fn parse_while_boolean<B: Fn(&i128, &i128) -> bool, F: Fn(&ParseReturnInfo) -> bool>(
    position: (&[u8], usize),
    boolean: B,
    condition: F,
) -> ParseReturnInfo {
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
        return_info.value = boolean(&return_info.value, &returned.value) as i128;
    }
    return_info
}

pub fn parse_operator(operator: PacketType, position: (&[u8], usize)) -> ParseReturnInfo {
    let bit_len_operator = BitLenOperator::from_bytes(position);
    let packet_count_operator = PacketNumOperator::from_bytes(position);

    let header_len: usize;
    let condition: Box<dyn Fn(&ParseReturnInfo) -> bool>;
    let position;

    if let Ok((_position, bit_len_operator)) = bit_len_operator {
        header_len = 16;
        condition = Box::new(move |return_info: &ParseReturnInfo| {
            return_info.bits_read < bit_len_operator.bits as usize
        });
        position = _position;
    } else if let Ok((_position, packet_count_operator)) = packet_count_operator {
        header_len = 12;
        condition = Box::new(move |return_info: &ParseReturnInfo| {
            return_info.packets_read < packet_count_operator.packets as usize
        });
        position = _position;
    } else {
        panic!("WTF");
    }

    let return_info = match operator {
        PacketType::Sum => parse_while_calculative(position, i128::checked_add, condition),
        PacketType::Product => parse_while_calculative(position, i128::checked_mul, condition),
        PacketType::Minimum => parse_while_selective(position, i128::min, condition),
        PacketType::Maximum => parse_while_selective(position, i128::max, condition),
        PacketType::GreaterThan => parse_while_boolean(position, i128::gt, condition),
        PacketType::LessThan => parse_while_boolean(position, i128::lt, condition),
        PacketType::Equal => parse_while_boolean(position, i128::eq, condition),
        _ => panic!("WTF2"),
    };

    ParseReturnInfo {
        position: return_info.position,
        bits_read: header_len + return_info.bits_read,
        packets_read: return_info.packets_read,
        value: return_info.value,
    }
}
