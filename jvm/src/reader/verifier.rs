use crate::primitive_enum;

primitive_enum!(u16,
    #[allow(non_camel_case_types)]
    Verifier {
        INVOKED_DYNAMIC_MAJOR_VERSION = 51,
        DYNAMIC_CONSTANT_MAJOR_VERSION = 55,
    }
);
