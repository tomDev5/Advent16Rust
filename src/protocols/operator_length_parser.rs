use bitvec::{field::BitField, order::Msb0, slice::BitSlice};
use deku::DekuError;

pub fn parse_operator_length(
    rest: &BitSlice<Msb0, u8>,
    required_length_type: bool,
    length_size: usize,
) -> Result<(&BitSlice<Msb0, u8>, u16), DekuError> {
    let length_type = rest.get(0);
    if length_type.is_none() {
        return Err(DekuError::InvalidParam("Not enough data".to_string()));
    }
    let length_type = length_type.unwrap();

    if *length_type != required_length_type {
        return Err(DekuError::InvalidParam("Not a valid operator".to_string()));
    }

    let bit_count = rest.get(1..length_size + 1);
    if bit_count.is_none() {
        return Err(DekuError::InvalidParam(
            "Not enough data for required length".to_string(),
        ));
    }
    let bit_count = bit_count.unwrap().load_be::<u16>();

    Ok((rest.get(length_size + 1..).unwrap(), bit_count))
}
