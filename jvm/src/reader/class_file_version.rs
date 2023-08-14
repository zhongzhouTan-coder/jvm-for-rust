#[repr(u16)]
#[derive(Debug)]
pub enum ClassFileVersion {
    Jdk1_1,
    Jdk1_2,
    Jdk1_3,
    Jdk1_4,
    Jdk5_0,
    Jdk6,
    Jdk7,
    Jdk8,
    Jdk9,
    Jdk10,
    Jdk11,
    Jdk12,
    Jdk13,
    Jdk14,
    Jdk15,
    Jdk16,
    Jdk17,
}

impl Default for ClassFileVersion {
    fn default() -> Self {
        ClassFileVersion::Jdk11
    }
}

impl ClassFileVersion {
    pub fn from_u16(version: u16) -> ClassFileVersion {
        match version {
            45 => Self::Jdk1_1,
            46 => Self::Jdk1_2,
            47 => Self::Jdk1_3,
            48 => Self::Jdk1_4,
            49 => Self::Jdk5_0,
            50 => Self::Jdk6,
            51 => Self::Jdk7,
            52 => Self::Jdk8,
            53 => Self::Jdk9,
            54 => Self::Jdk10,
            55 => Self::Jdk11,
            56 => Self::Jdk12,
            57 => Self::Jdk13,
            58 => Self::Jdk14,
            59 => Self::Jdk15,
            60 => Self::Jdk16,
            61 => Self::Jdk17,
            _ => panic!("unsupported jdk version."),
        }
    }
}
