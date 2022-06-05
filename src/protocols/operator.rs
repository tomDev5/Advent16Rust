use deku::DekuContainerRead;

use crate::{
    parse_packet,
    protocols::{
        bit_len_operator::BitLenOperator, packet_num_operator::PacketNumOperator,
        return_info::ParseReturnInfo,
    },
};

use super::base::PacketType;

fn parse_while<F: Fn(&ParseReturnInfo) -> bool>(
    position: (&[u8], usize),
    condition: F,
) -> ParseReturnInfo {
    let mut return_info = ParseReturnInfo {
        position,
        bits_read: 0,
        packets_read: 0,
        value: 0,
    };
    while condition(&return_info) {
        let returned = parse_packet(return_info.position);
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += 1;
        return_info.value += returned.value; // depend on action
    }
    return_info
}

pub fn parse_operator(_operator: PacketType, position: (&[u8], usize)) -> ParseReturnInfo {
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

    let return_info = parse_while(position, condition);
    ParseReturnInfo {
        position: return_info.position,
        bits_read: header_len + return_info.bits_read,
        packets_read: return_info.packets_read,
        value: return_info.value,
    }
}
