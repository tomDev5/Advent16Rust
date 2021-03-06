use crate::protocols::return_info::ParseReturnInfo;
use bitvec::{field::BitField, order::Msb0, slice::BitSlice};
use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(endian = "big")]
pub struct Literal {
    #[deku(reader = "Literal::read(deku::rest)")]
    pub content: (u128, usize),
}

impl Literal {
    /**
     * This method is the deserializer
     * PARAM:
     *      rest - bitslice of the data to parse
     * RETURN:
     *      On success, a tuple of the remaining data, and the result literal
     *      On failure - An error
     */
    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, (u128, usize)), DekuError> {
        let mut result: u128 = 0;
        let mut index = 0;
        loop {
            let is_last = !rest.get(index).ok_or(DekuError::InvalidParam(
                "Not enough data for is_last".to_owned(),
            ))?;
            let nibble = rest
                .get(index + 1..index + 5)
                .ok_or(DekuError::InvalidParam(
                    "Not enough data to nibble".to_owned(),
                ))?;

            result = (result << 4) + nibble.load_be::<u128>();
            index += 5;
            if is_last {
                break;
            }
        }
        Ok((
            rest.get(index..).ok_or(DekuError::InvalidParam(
                "Not enough data for position".to_owned(),
            ))?,
            (result, index),
        ))
    }
}

pub fn parse_literal(position: (&[u8], usize)) -> Result<ParseReturnInfo, DekuError> {
    let (position, literal) = Literal::from_bytes(position)?;
    Ok(ParseReturnInfo {
        position,
        bits_read: literal.content.1,
        packets_read: 1,
        value: literal.content.0 as i128,
    })
}
