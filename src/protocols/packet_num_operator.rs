use crate::protocols::operator_length_parser::parse_operator_length;
use bitvec::{order::Msb0, slice::BitSlice};
use deku::{DekuContainerRead, DekuError, DekuRead};

#[derive(Debug, PartialEq, DekuRead)]
#[deku(endian = "big")]
pub struct PacketNumOperator {
    #[deku(bits = "11", reader = "PacketNumOperator::read(deku::rest)")]
    pub packets: u16,
}

impl PacketNumOperator {
    /**
     * This method is the deserializer
     * PARAM:
     *      rest - bitslice of the data to parse
     * RETURN:
     *      On success, a tuple of the remaining data, and the result literal
     *      On failure - An error
     */
    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, u16), DekuError> {
        parse_operator_length(rest, true, 11)
    }
}
