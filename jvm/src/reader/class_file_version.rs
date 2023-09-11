use crate::primitive_enum;

primitive_enum!(
    u16,
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    ClassFileVersion {
        Jdk1_1 = 45,
        Jdk1_2 = 46,
        Jdk1_3 = 47,
        Jdk1_4 = 48,
        Jdk5_0 = 49,
        Jdk6 = 50,
        Jdk7 = 51,
        Jdk8 = 52,
        Jdk9 = 53,
        Jdk10 = 54,
        Jdk11 = 55,
        Jdk12 = 56,
        Jdk13 = 57,
        Jdk14 = 58,
        Jdk15 = 59,
        Jdk16 = 60,
        Jdk17 = 61,
    }
);

impl Default for ClassFileVersion {
    fn default() -> Self {
        ClassFileVersion::Jdk11
    }
}
