use crate::primitive_enum;

primitive_enum!(
    u16,
    #[allow(non_camel_case_types)]
    ReferenceKind {
        JVM_REF_getField = 1,
        JVM_REF_getStatic = 2,
        JVM_REF_putField = 3,
        JVM_REF_putStatic = 4,
        JVM_REF_invokeVirtual = 5,
        JVM_REF_newInvokeSpecial = 6,
        JVM_REF_invokeStatic = 7,
        JVM_REF_invokeSpecial = 8,
        JVM_REF_invokeInterface = 9,
    }
);
