#[derive(Debug)]
pub struct ParseReturnInfo<'a> {
    pub position: (&'a [u8], usize),
    pub bits_read: usize,
    pub packets_read: usize,
    pub value: i128,
}

impl ParseReturnInfo<'_> {
    pub fn add_bits(self, bits: usize) -> Self {
        ParseReturnInfo {
            bits_read: self.bits_read + bits,
            ..self
        }
    }
}
