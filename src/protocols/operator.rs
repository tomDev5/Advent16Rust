use deku::DekuContainerRead;

use crate::{
    parse_packet,
    protocols::{
        bit_len_operator::BitLenOperator, packet_num_operator::PacketNumOperator,
        return_info::ParseReturnInfo,
    },
};

fn parse_while<F: Fn(&ParseReturnInfo) -> bool>(
    position: (&[u8], usize),
    condition: F,
) -> ParseReturnInfo {
    let mut return_info = ParseReturnInfo {
        position,
        bits_read: 0,
        packets_read: 0,
    };
    while condition(&return_info) {
        let returned = parse_packet(return_info.position);
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += returned.packets_read;
    }
    return_info
}

pub fn parse_operator(position: (&[u8], usize)) -> ParseReturnInfo {
    println!("Parsing Operator");
    let bit_len_operator = BitLenOperator::from_bytes(position);
    let packet_count_operator = PacketNumOperator::from_bytes(position);

    if let Ok((position, bit_len_operator)) = bit_len_operator {
        let bit_len = bit_len_operator.bits;
        println!("Length in bits: {}", bit_len);

        parse_while(position, |return_info: &ParseReturnInfo| {
            return_info.bits_read < bit_len as usize
        })
    } else if let Ok((position, packet_count_operator)) = packet_count_operator {
        let packet_count = packet_count_operator.packets;
        println!("Number of packets: {}", packet_count);

        parse_while(position, |return_info: &ParseReturnInfo| {
            return_info.packets_read < packet_count as usize
        })
    } else {
        panic!("WTF");
    }
}
