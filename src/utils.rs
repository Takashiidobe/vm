pub(crate) fn u16_to_u8(double: u16) -> [u8; 2] {
    double.to_ne_bytes()
}

pub(crate) fn i16_to_u8(double: i16) -> [u8; 2] {
    double.to_ne_bytes()
}

pub(crate) fn u8_to_u16(b1: u8, b2: u8) -> u16 {
    u16::from_ne_bytes([b1, b2])
}

pub(crate) fn u8_to_i16(b1: u8, b2: u8) -> i16 {
    i16::from_ne_bytes([b1, b2])
}

pub const REGISTER_COUNT: usize = 16;
pub const STACK_SIZE: usize = 65536;
