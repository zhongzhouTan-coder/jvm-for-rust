use std::ops;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum ClassAccessFlag {
    PUBLIC = 0x0001,
    FINAL = 0x0010,
    SUPER = 0x0020,
    INTERFACE = 0x0200,
    ABSTRACT = 0x0400,
    SYNTHETIC = 0x1000,
    ANNOTATION = 0x2000,
    ENUM = 0x4000,
    MODULE = 0x8000,
}

impl Default for ClassAccessFlag {
    fn default() -> Self {
        Self::PUBLIC
    }
}

impl ops::BitOr for ClassAccessFlag {
    type Output = u16;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u16 | rhs as u16
    }
}

impl PartialEq<u16> for ClassAccessFlag {
    fn eq(&self, other: &u16) -> bool {
        *self as u16 == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::class_access_flag::ClassAccessFlag;

    #[test]
    fn we_can_compare_access_flag_with_u16() {
        let final_flag = 0x0010u16;
        let public_flag = 0x0001u16;
        assert_eq!(ClassAccessFlag::FINAL, final_flag);
        assert_eq!(ClassAccessFlag::PUBLIC, public_flag);
    }

    #[test]
    fn we_can_do_bit_or_operation() {
        let access_flag = 0x0201;
        assert_eq!(
            ClassAccessFlag::PUBLIC | ClassAccessFlag::INTERFACE,
            access_flag
        );
    }
}
