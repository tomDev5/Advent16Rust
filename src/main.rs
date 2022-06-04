mod protocols;

use deku::prelude::*;
use protocols::return_info::ParseReturnInfo;
use std::num::ParseIntError;

use std::fs;

use crate::protocols::{base::PacketBase, literal::parse_literal, operator::parse_operator};

fn main() {
    if let Ok(data) = fs::read_to_string("./input/input.txt") {
        let data = decode_hex(data.trim()).unwrap();
        solve_part_one(&data);
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn solve_part_one(bytes: &[u8]) {
    let result = parse_packet((bytes, 0));
    println!("parsed packet: {:?}", result);
}

fn parse_packet(position: (&[u8], usize)) -> ParseReturnInfo {
    let (position, base) = PacketBase::from_bytes(position).unwrap();
    println!("{:?}", base);

    let return_info = match base.next_proto {
        4 => parse_literal(position),
        _ => parse_operator(position),
    };

    ParseReturnInfo {
        position: return_info.position,
        bits_read: 6 + return_info.bits_read,
        packets_read: return_info.packets_read,
    }
}
