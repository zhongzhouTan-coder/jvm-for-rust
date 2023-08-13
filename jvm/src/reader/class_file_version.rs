#[repr(u16)]
#[derive(Debug)]
pub enum ClassFileVersion {
    JDK_1_1 = 45,
    JDK_1_2 = 46,
    JDK_1_3 = 47,
    JDK_1_4 = 48,
    JDK_5_0 = 49,
    JDK_6 = 50,
    JDK_7 = 51,
    JDK_8 = 52,
    JDK_9 = 53,
    JDK_10 = 54,
    JDK_11 = 55,
    JDK_12 = 56,
    JDK_13 = 57,
    JDK_14 = 58,
    JDK_15 = 59,
    JDK_16 = 60,
    JDK_17 = 61,
}

impl Default for ClassFileVersion {
    fn default() -> Self {
        ClassFileVersion::JDK_11
    }
}

impl ClassFileVersion {
    pub fn from_u16(version: u16) -> ClassFileVersion {
        match version {
            45 => Self::JDK_1_1,
            46 => Self::JDK_1_2,
            47 => Self::JDK_1_3,
            48 => Self::JDK_1_4,
            49 => Self::JDK_5_0,
            50 => Self::JDK_6,
            51 => Self::JDK_7,
            52 => Self::JDK_8,
            53 => Self::JDK_9,
            54 => Self::JDK_10,
            55 => Self::JDK_11,
            56 => Self::JDK_12,
            57 => Self::JDK_13,
            58 => Self::JDK_14,
            59 => Self::JDK_15,
            60 => Self::JDK_16,
            61 => Self::JDK_17,
            _ => panic!("unsupported jdk version."),
        }
    }
}
