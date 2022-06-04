#[derive(Debug)]
pub struct ParseReturnInfo<'a> {
    pub position: (&'a [u8], usize),
    pub bits_read: usize,
    pub packets_read: usize,
}
