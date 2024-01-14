pub fn check_cross_page(base: u16, offset: u8) -> bool {
    high_byte(base + offset as u16) != high_byte(base)
}

pub fn offset<T: Into<u16>>(base: T, offset: u8) -> u16 {
    base.into() + offset as u16
}

pub fn low_byte<T: Into<u16>>(value: T) -> u16 {
    value.into() & 0xFF
}

pub fn high_byte(value: u16) -> u16 {
    value & 0xFF00
}
