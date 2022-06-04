use bitvec::{order::Msb0, slice::BitSlice};
use deku::{DekuContainerRead, DekuError, DekuRead};

use super::operator_length_parser::parse_operator_length;

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "big")]
pub struct BitLenOperator {
    #[deku(bits = "15", reader = "BitLenOperator::read(deku::rest)")]
    pub bits: u16,
}

impl BitLenOperator {
    /**
     * This method is the deserializer
     * PARAM:
     *      rest - bitslice of the data to parse
     * RETURN:
     *      On success, a tuple of the remaining data, and the result literal
     *      On failure - An error
     */
    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, u16), DekuError> {
        parse_operator_length(rest, false, 15)
    }
}
