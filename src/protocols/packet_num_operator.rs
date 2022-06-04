use bitvec::{field::BitField, order::Msb0, slice::BitSlice};
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
        let length_type = rest.get(0);
        if length_type.is_none() {
            return Err(DekuError::InvalidParam("Not enough data".to_string()));
        }
        let length_type = length_type.unwrap();

        if *length_type != true {
            return Err(DekuError::InvalidParam(
                "Not a packet number operator".to_string(),
            ));
        }

        let bit_count = rest.get(1..12);
        if bit_count.is_none() {
            return Err(DekuError::InvalidParam(
                "Not enough data for packet count".to_string(),
            ));
        }
        let bit_count = bit_count.unwrap().load_be::<u16>();

        Ok((rest.get(12..).unwrap(), bit_count))
    }
}
