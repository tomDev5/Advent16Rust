use crate::{protocols::base::parse_packet_from_base, protocols::return_info::ParseReturnInfo};
use deku::DekuContainerRead;
use deku::DekuEnumExt;
use deku::{DekuError, DekuRead};

const BIT_LENGTH_LENGTH: usize = 15;
const PACKET_NUMBER_LENGTH: usize = 11;

#[derive(Debug, DekuRead)]
#[deku(
    type = "u8",
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    bits = "3"
)]
pub enum Operator {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

impl Operator {
    fn to_action(&self) -> Box<dyn Fn(i128, i128) -> i128> {
        match self {
            Operator::Sum => Box::new(|lhs, rhs| lhs + rhs),
            Operator::Product => Box::new(|lhs, rhs| lhs * rhs),
            Operator::Minimum => Box::new(|lhs, rhs| lhs.min(rhs)),
            Operator::Maximum => Box::new(|lhs, rhs| lhs.max(rhs)),
            Operator::GreaterThan => Box::new(|lhs, rhs| i128::from(lhs > rhs)),
            Operator::LessThan => Box::new(|lhs, rhs| i128::from(lhs < rhs)),
            Operator::Equal => Box::new(|lhs, rhs| i128::from(lhs == rhs)),
        }
    }
}

#[derive(Debug, DekuRead)]
#[deku(endian = "big")]
#[deku(type = "u16", bits = "1")]
enum LengthOperator {
    #[deku(id = "0")]
    BitLengthOperator(#[deku(bits = "15")] u16),
    #[deku(id = "1")]
    PacketNumberOperator(#[deku(bits = "11")] u16),
}

impl LengthOperator {
    fn to_condition(self) -> (Box<dyn Fn(&ParseReturnInfo) -> bool>, usize) {
        match self {
            LengthOperator::BitLengthOperator(bits) => (
                Box::new(move |return_info| return_info.bits_read < bits as usize),
                BIT_LENGTH_LENGTH + 1,
            ),
            LengthOperator::PacketNumberOperator(packets) => (
                Box::new(move |return_info| return_info.packets_read < packets as usize),
                PACKET_NUMBER_LENGTH + 1,
            ),
        }
    }
}

pub fn parse_operator(
    operator: Operator,
    position: (&[u8], usize),
) -> Result<ParseReturnInfo, DekuError> {
    let (position, length_operator) = LengthOperator::from_bytes(position)?;
    let (condition, header_len) = length_operator.to_condition();
    perform_operator_action(operator, header_len, position, condition)
}

fn perform_operator_action(
    operator: Operator,
    header_len: usize,
    position: (&[u8], usize),
    condition: impl Fn(&ParseReturnInfo) -> bool,
) -> Result<ParseReturnInfo, DekuError> {
    let return_info = parse_subpackets(position, operator.to_action(), condition)?;
    Ok(return_info.add_bits(header_len))
}

fn parse_subpackets(
    position: (&[u8], usize),
    action: impl Fn(i128, i128) -> i128,
    condition: impl Fn(&ParseReturnInfo) -> bool,
) -> Result<ParseReturnInfo, DekuError> {
    let mut return_info = parse_packet_from_base(position)?;
    while condition(&return_info) {
        let returned = parse_packet_from_base(return_info.position)?;
        return_info.position = returned.position;
        return_info.bits_read += returned.bits_read;
        return_info.packets_read += 1;
        return_info.value = action(return_info.value, returned.value);
    }
    Ok(return_info)
}
