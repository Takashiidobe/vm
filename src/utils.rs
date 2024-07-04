pub(crate) fn u16_to_u8(double: u16) -> [u8; 2] {
    double.to_ne_bytes()
}

pub(crate) fn u8_to_u16(b1: u8, b2: u8) -> u16 {
    ((b2 as u16) << 8) | b1 as u16
}

pub const REGISTER_COUNT: usize = 16;
pub const STACK_SIZE: usize = 65536;
