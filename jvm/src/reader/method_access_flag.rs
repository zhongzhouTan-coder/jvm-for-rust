use std::ops;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum MethodAccessFlag {
    PUBLIC = 0x0001,
    PRIVATE = 0x0002,
    PROTECTED = 0x0004,
    STATIC = 0x0008,
    FINAL = 0x0010,
    SYNCHRONIZED = 0x0020,
    BRIDGE = 0x0040,
    VARARGS = 0x0080,
    NATIVE = 0x0100,
    ABSTRACT = 0x0400,
    STRICT = 0x0800,
    SYNTHETIC = 0x1000,
}

impl MethodAccessFlag {
    pub fn from_u16(flag: u16) -> Self {
        match flag {
            0x0001u16 => Self::PUBLIC,
            0x0002u16 => Self::PRIVATE,
            0x0004u16 => Self::PROTECTED,
            0x0008u16 => Self::STATIC,
            0x0010u16 => Self::FINAL,
            0x0020u16 => Self::SYNCHRONIZED,
            0x0040u16 => Self::BRIDGE,
            0x0080u16 => Self::VARARGS,
            0x0100u16 => Self::NATIVE,
            0x0400u16 => Self::ABSTRACT,
            0x0800u16 => Self::STRICT,
            0x1000u16 => Self::SYNTHETIC,
            _ => panic!("Invalid method flag - {}", flag),
        }
    }
}

impl Default for MethodAccessFlag {
    fn default() -> Self {
        Self::PUBLIC
    }
}

impl PartialEq<u16> for MethodAccessFlag {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

impl ops::BitOr for MethodAccessFlag {
    type Output = u16;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u16 | rhs as u16
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::method_access_flag::MethodAccessFlag;

    #[test]
    fn test_eq_for_access_flag() {
        let public_flag = 0x0001u16;
        assert_eq!(MethodAccessFlag::PUBLIC, public_flag);
    }

    #[test]
    fn test_bit_or_access_flag() {
        let flag = MethodAccessFlag::PUBLIC | MethodAccessFlag::STRICT;
        assert_eq!(flag, 0x0801u16);
    }
}
