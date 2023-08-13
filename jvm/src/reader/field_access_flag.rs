use std::ops;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum FieldAccessFlag {
    PUBLIC = 0x0001,
    PRIVATE = 0x0002,
    PROTECTED = 0x0004,
    STATIC = 0x0008,
    FINAL = 0x0010,
    VOLATILE = 0x0040,
    TRANSIENT = 0x1000,
    ENUM = 0x4000,
}

impl FieldAccessFlag {
    pub fn from_u16(flag: u16) -> Self {
        match flag {
            0x0001u16 => Self::PUBLIC,
            0x0002u16 => Self::PRIVATE,
            0x0004u16 => Self::PROTECTED,
            0x0008u16 => Self::STATIC,
            0x0010u16 => Self::FINAL,
            0x0040u16 => Self::VOLATILE,
            0x1000u16 => Self::TRANSIENT,
            0x4000u16 => Self::ENUM,
            _ => panic!("Invalid field access flag - {}.", flag),
        }
    }
}

impl Default for FieldAccessFlag {
    fn default() -> Self {
        FieldAccessFlag::PUBLIC
    }
}

impl PartialEq<u16> for FieldAccessFlag {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl ops::BitOr for FieldAccessFlag {
    type Output = u16;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u16 | rhs as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::field_access_flag::FieldAccessFlag;

    #[test]
    fn we_can_compare_flag_with_u16_type() {
        let flag = 0x0001u16;
        assert_eq!(FieldAccessFlag::PUBLIC, flag);
    }

    #[test]
    fn we_can_do_bit_or_operation_for_flag() {
        let result = FieldAccessFlag::PUBLIC | FieldAccessFlag::STATIC;
        assert_eq!(result, 0x0009u16);
    }
}
