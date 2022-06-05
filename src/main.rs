mod protocols;

use crate::protocols::base::parse_packet_from_base;
use std::fs;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = fs::read_to_string("./input/input.txt")?;
    let lines = lines.lines();
    for line in lines {
        let data = decode_hex(line.trim())?;
        let result = parse_packet_from_base((data.as_ref(), 0));
        println!("{:?}", result?);
    }

    Ok(())
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
